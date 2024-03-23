use darling::ast::NestedMeta;
use darling::{Error, FromMeta};
use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, ExprArray, Ident, ItemFn, Token};

macro_rules! event_handler_args {
    [ $($event_name:ident => $event_ty:ident:$arg_type:ident),* $(,)? ] => {
        #[allow(dead_code)]
        #[derive(Debug, FromMeta)]
        struct EventHandlerArgs {
            $(
                #[darling(default)] $event_name: bool,
            )*
        }

        #[allow(dead_code)]
        impl EventHandlerArgs {
            /// Returns if only one of the events is turned on.
            pub fn only_one(&self) -> bool {
                [$(self.$event_name,)*]
                    .into_iter()
                    .filter(|b| *b)
                    .count()
                == 1
            }

            /// Returns the name of all the events.
            pub fn all_events(&self) -> &'static [&'static str] {
                &[$(stringify!($event_name),)*]
            }

            pub fn get(&self, fn_name: &str, param_name: &proc_macro2::TokenStream) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
                match () {
                    $(
                        _ if self.$event_name
                            || fn_name.to_lowercase() == stringify!($event_name).to_lowercase()
                        => (
                            quote! { descord::internals::HandlerValue::$arg_type(#param_name) },
                            quote! { descord::Event::$event_ty },
                        ),
                    )*

                    _ => panic!("Enable one of {:?} event", self.all_events()),
                }
            }

            pub fn check_arg(&self) {

            }
        }
    };
}

macro_rules! type_path {
    [ $ty:ident, $name:ident ] => {
        quote! { descord::internals::Value::$ty(ref #$name) }
    };
}

macro_rules! type_name {
    [ $ty:ident ] => {
        quote! { descord::internals::ParamType::$ty }
    };
}

#[rustfmt::ignore]
event_handler_args![
//  event switch       => event type         : event data type
    ready              => Ready              : ReadyData,
    message_create     => MessageCreate      : Message,
    message_delete     => MessageDelete      : Message,
    message_delete_raw => MessageDeleteRaw   : DeletedMessage,
    message_update     => MessageUpdate      : MessageData,
    reaction_add       => MessageReactionAdd : Reaction,
    guild_create       => GuildCreate        : GuildCreate,
    interaction_create => InteractionCreate  : Interaction,
];

#[derive(Debug, FromMeta)]
struct CommandArgs {
    #[darling(default)]
    name: Option<String>,
    #[darling(default)]
    prefix: Option<String>,
}

#[proc_macro_attribute]
pub fn event(args: TokenStream, input: TokenStream) -> TokenStream {
    let function = parse_macro_input!(input as ItemFn);
    let function_vis = function.vis;
    let function_name = &function.sig.ident;
    let function_body = &function.block;

    if function.sig.inputs.len() != 1 {
        panic!("Expected only one parameter");
    }

    let param_name = match function.sig.inputs.first().unwrap() {
        syn::FnArg::Typed(x) => match *x.pat {
            syn::Pat::Ident(ref ident) => quote! { #ident },
            syn::Pat::Wild(ref ident) => quote! { #ident },
            _ => panic!("unknown parameter name"),
        },
        _ => panic!("self???"),
    };

    if function.sig.asyncness.is_none() {
        panic!("Function marked with `#[descord::event(...)]` should be async");
    }

    let attr_args = match NestedMeta::parse_meta_list(args.into()) {
        Ok(v) => v,
        Err(e) => {
            return TokenStream::from(Error::from(e).write_errors());
        }
    };

    let handler_args: EventHandlerArgs = match EventHandlerArgs::from_list(&attr_args) {
        Ok(v) => v,
        Err(e) => {
            return TokenStream::from(Error::from(e).write_errors());
        }
    };

    let (name, event_ty) = handler_args.get(&function_name.to_string(), &param_name);

    let let_stmt = quote! {
        let #name = data else {
            unreachable!()
        };
    };

    let expanded = quote! {
        #function_vis fn #function_name() -> descord::internals::EventHandler {
            use descord::prelude::*;

            fn f(
                data: descord::internals::HandlerValue
            ) -> std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send + 'static>> {
                Box::pin(async move {
                    #let_stmt
                    #function_body
                })
            }

            internals::EventHandler {
                event: #event_ty,
                handler_fn: f,
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_attribute]
pub fn command(args: TokenStream, input: TokenStream) -> TokenStream {
    let function = parse_macro_input!(input as ItemFn);

    if function.sig.asyncness.is_none() {
        panic!("Function marked with `#[descord::command(...)]` should be async");
    }

    let attr_args = match NestedMeta::parse_meta_list(args.into()) {
        Ok(v) => v,
        Err(e) => {
            return TokenStream::from(Error::from(e).write_errors());
        }
    };

    let command_args: CommandArgs = match CommandArgs::from_list(&attr_args) {
        Ok(v) => v,
        Err(e) => {
            return TokenStream::from(Error::from(e).write_errors());
        }
    };

    let custom_prefix = command_args.prefix.is_some();
    let new_name = format!(
        "{}{}",
        command_args.prefix.as_ref().unwrap_or(&String::new()),
        command_args
            .name
            .unwrap_or_else(|| function.sig.ident.to_string())
    );

    let function_name = &function.sig.ident;
    let function_body = &function.block;
    let function_params = &function.sig.inputs;
    let function_vis = function.vis;

    let error = || -> ! { panic!("Expected `descord::prelude::Message` as the first argument") };
    let first_param_name = match function_params.first() {
        Some(param) => {
            let param = match param {
                syn::FnArg::Typed(x) => x,
                _ => panic!("`self` is not allowed"),
            };

            match *param.ty {
                syn::Type::Path(ref path) if path.path.is_ident("Message") => {}
                _ => error(),
            }

            match *param.pat {
                syn::Pat::Ident(ref ident) => quote! { #ident },
                syn::Pat::Wild(ref ident) => quote! { #ident },
                _ => panic!("unknown param name"),
            }
        }

        _ => error(),
    };

    let mut param_types = vec![];
    let mut stmts: Vec<proc_macro2::TokenStream> = vec![];

    let mut stop = false;
    for (idx, param) in function_params.iter().skip(1).enumerate() {
        if stop {
            panic!("`Arg` should be the last parameter");
        }

        let param = match param {
            syn::FnArg::Typed(x) => x,
            _ => panic!("`self` is not allowed"),
        };

        let syn::Pat::Ident(name) = &*param.pat else {
            panic!();
        };

        let type_ = (*param.ty).clone();

        let syn::Type::Path(path) = type_ else {
            panic!("Expected a path found something else");
        };

        let (name, ty) = match path
            .path
            .segments
            .last()
            .unwrap()
            .ident
            .to_string()
            .as_str()
        {
            "String" => (type_path!(String, name), type_name!(String)),
            "isize" => (type_path!(Int, name), type_name!(Int)),
            "bool" => (type_path!(Bool, name), type_name!(Bool)),
            "Channel" => (type_path!(Channel, name), type_name!(Channel)),
            "User" => (type_path!(User, name), type_name!(User)),
            "Args" => {
                stop = true; // will stop the loop from running again
                (type_path!(Args, name), type_name!(Args))
            }

            _ => panic!("Unsupported type"),
        };

        param_types.push(ty);
        stmts.push(quote! {
            let #name = args[#idx].clone() else { unreachable!() };
        });
    }

    let mut let_stmts = proc_macro2::TokenStream::new();
    let_stmts.extend(stmts.into_iter());

    let expanded = quote! {
        #function_vis fn #function_name() -> descord::internals::Command {
            use descord::prelude::*;

            fn f(
                #first_param_name: Message,
                args: Vec<internals::Value>
            ) -> std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send + 'static>> {
                Box::pin(async move {
                    #let_stmts
                    drop(args);

                    #function_body
                })
            }

            internals::Command {
                name: String::from(#new_name),
                fn_sig: vec![#(#param_types),*],
                handler_fn: f,
                custom_prefix: #custom_prefix,
            }
        }
    };

    TokenStream::from(expanded)
}

#[derive(Debug, FromMeta)]
struct SlashCommandArgs {
    #[darling(default)]
    name: Option<String>,
    #[darling(default)]
    description: Option<String>,
}

#[proc_macro_attribute]
pub fn slash(args: TokenStream, input: TokenStream) -> TokenStream {
    let function = parse_macro_input!(input as ItemFn);

    if function.sig.asyncness.is_none() {
        panic!("Function marked with `#[descord::slash(...)]` should be async");
    }

    let attr_args = match NestedMeta::parse_meta_list(args.into()) {
        Ok(v) => v,
        Err(e) => {
            return TokenStream::from(Error::from(e).write_errors());
        }
    };

    let slash_command_args: SlashCommandArgs = match SlashCommandArgs::from_list(&attr_args) {
        Ok(v) => v,
        Err(e) => {
            return TokenStream::from(Error::from(e).write_errors());
        }
    };

    let new_name = slash_command_args
        .name
        .unwrap_or_else(|| function.sig.ident.to_string());

    let description = slash_command_args
        .description
        .unwrap_or_else(|| String::from("No description provided"));

    let function_name = &function.sig.ident;
    let function_body = &function.block;
    let function_params = &function.sig.inputs;
    let function_vis = function.vis;

    let error = || -> ! { panic!("Expected `descord::prelude::Interaction` as the first argument") };
    let first_param_name = match function_params.first() {
        Some(param) => {
            let param = match param {
                syn::FnArg::Typed(x) => x,
                _ => panic!("`self` is not allowed"),
            };

            match *param.ty {
                syn::Type::Path(ref path) if path.path.is_ident("Interaction") => {}
                _ => error(),
            }

            match *param.pat {
                syn::Pat::Ident(ref ident) => quote! { #ident },
                syn::Pat::Wild(ref ident) => quote! { #ident },
                _ => panic!("unknown param name"),
            }
        }

        _ => error(),
    };

    let mut param_types = vec![];
    let mut param_names = vec![];
    let mut param_descriptions = vec![];
    let mut stmts: Vec<proc_macro2::TokenStream> = vec![];

    let stop = false;
    for (idx, param) in function_params.iter().skip(1).enumerate() {
        if stop {
            panic!("`Arg` should be the last parameter");
        }

        let param = match param {
            syn::FnArg::Typed(x) => x,
            _ => panic!("`self` is not allowed"),
        };

        let syn::Pat::Ident(name) = &*param.pat else {
            panic!();
        };

        param_names.push(quote! { stringify!(#name).to_string() });

        // Extract the description from the comment
        let description = param.attrs.iter()
            .filter_map(|attr| {
                if let syn::Meta::NameValue(nv) = &attr.meta {
                    if nv.path.is_ident("doc") {
                        if let syn::Expr::Lit(lit) = &nv.value {
                            if let syn::Lit::Str(lit_str) = &lit.lit {
                                return Some(lit_str.token().to_string().trim_matches('"').trim().to_string());
                            }
                        }
                    }
                }
                None
            })
            .next()
            .unwrap_or_else(|| String::from("No description provided"));

        param_descriptions.push(description);
        let type_ = (*param.ty).clone();

        let syn::Type::Path(path) = type_ else {
            panic!("Expected a path found something else");
        };

        let (name, ty) = match path
            .path
            .segments
            .last()
            .unwrap()
            .ident
            .to_string()
            .as_str()
        {
            "String" => (type_path!(String, name), type_name!(String)),
            "isize" => (type_path!(Int, name), type_name!(Int)),
            "bool" => (type_path!(Bool, name), type_name!(Bool)),
            "Channel" => (type_path!(Channel, name), type_name!(Channel)),
            "User" => (type_path!(User, name), type_name!(User)),

            _ => panic!("Unsupported type"),
        };

        param_types.push(ty);
        stmts.push(quote! {
            let #name = args[#idx].clone() else { unreachable!() };
        });
    }

    let mut let_stmts = proc_macro2::TokenStream::new();
    let_stmts.extend(stmts.into_iter());

    let expanded = quote! {
        #function_vis fn #function_name() -> descord::internals::SlashCommand {
            use descord::prelude::*;

            fn f(
                #first_param_name: descord::models::interaction::Interaction,
                args: Vec<internals::Value>
            ) -> std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send + 'static>> {
                Box::pin(async move {
                    #let_stmts
                    drop(args);

                    #function_body
                })
            }

            internals::SlashCommand {
                name: String::from(#new_name),
                description: String::from(#description),
                fn_sig: vec![#(#param_types),*],
                fn_param_names: vec![#(#param_names),*],
                fn_param_descriptions: vec![#(#param_descriptions.to_string()),*],
                handler_fn: f,
            }
        }
    };

    TokenStream::from(expanded)
}

/// Usage: `register_all_commands!(client => ["src/commands.rs", "src/commands2.rs"]);`
/// Where `client` is the client object and the array is the list of files to search for commands.
/// If the array is empty, it will recursively search for files in the `src` directory.
#[proc_macro]
pub fn register_all_commands(input: TokenStream) -> TokenStream {
    let RegisterCmd {
        client_obj,
        file_array,
    } = parse_macro_input!(input as RegisterCmd);

    let paths: Vec<String> = if !file_array.elems.is_empty() {
        file_array
            .elems
            .into_iter()
            .map(|elem| {
                if let syn::Expr::Lit(lit) = elem {
                    if let syn::Lit::Str(lit_str) = lit.lit {
                        return lit_str.value();
                    }
                }
                panic!("Invalid expression provided");
            })
            .collect()
    } else {
        let mut paths = Vec::new();
        for entry in walkdir::WalkDir::new("src") {
            let entry = entry.unwrap();
            if entry.file_type().is_file() {
                paths.push(entry.path().to_string_lossy().into_owned());
            }
        }
        paths
    };

    let mut commands = Vec::new();

    for path in &paths {
        let items = syn::parse_file(&std::fs::read_to_string(&path).unwrap())
            .unwrap()
            .items;

        for item in items {
            if let syn::Item::Fn(function) = item {
                if function.attrs.iter().any(|attr| {
                    attr.path()
                        .segments
                        .last()
                        .map_or(false, |seg| seg.ident == "command")
                }) {
                    commands.push(function.sig.ident.clone());
                }
            }
        }
    }

    let expanded = quote! {
        #client_obj.register_commands(vec![#(#commands()),*]);
    };

    TokenStream::from(expanded)
}

#[proc_macro]
pub fn register_all_slash_commands(input: TokenStream) -> TokenStream {
    let RegisterCmd {
        client_obj,
        file_array,
    } = parse_macro_input!(input as RegisterCmd);

    let paths: Vec<String> = if !file_array.elems.is_empty() {
        file_array
            .elems
            .into_iter()
            .map(|elem| {
                if let syn::Expr::Lit(lit) = elem {
                    if let syn::Lit::Str(lit_str) = lit.lit {
                        return lit_str.value();
                    }
                }
                panic!("Invalid expression provided");
            })
            .collect()
    } else {
        let mut paths = Vec::new();
        for entry in walkdir::WalkDir::new("src") {
            let entry = entry.unwrap();
            if entry.file_type().is_file() {
                paths.push(entry.path().to_string_lossy().into_owned());
            }
        }
        paths
    };

    let mut commands = Vec::new();

    for path in &paths {
        let items = syn::parse_file(&std::fs::read_to_string(&path).unwrap())
            .unwrap()
            .items;

        for item in items {
            if let syn::Item::Fn(function) = item {
                if function.attrs.iter().any(|attr| {
                    attr.path()
                        .segments
                        .last()
                        .map_or(false, |seg| seg.ident == "slash")
                }) {
                    commands.push(function.sig.ident.clone());
                }
            }
        }
    }

    let expanded = quote! {
        #client_obj.register_slash_commands(vec![#(#commands()),*]).await;
    };

    TokenStream::from(expanded)
}

/// Usage: `register_all_events!(client => ["src/events.rs", "src/events2.rs"]);`
/// Where `client` is the client object and the array is the list of files to search for events.
/// If the array is empty, it will recursively search for files in the `src` directory.
#[proc_macro]
pub fn register_all_events(input: TokenStream) -> TokenStream {
    let RegisterCmd {
        client_obj,
        file_array,
    } = parse_macro_input!(input as RegisterCmd);

    let paths: Vec<String> = if !file_array.elems.is_empty() {
        file_array
            .elems
            .into_iter()
            .map(|elem| {
                if let syn::Expr::Lit(lit) = elem {
                    if let syn::Lit::Str(lit_str) = lit.lit {
                        return lit_str.value();
                    }
                }
                panic!("Invalid expression provided");
            })
            .collect()
    } else {
        let mut paths = Vec::new();
        for entry in walkdir::WalkDir::new("src") {
            let entry = entry.unwrap();
            if entry.file_type().is_file() {
                paths.push(entry.path().to_string_lossy().into_owned());
            }
        }
        paths
    };

    let mut events = Vec::new();

    for path in &paths {
        let items = syn::parse_file(&std::fs::read_to_string(&path).unwrap())
            .unwrap()
            .items;

        for item in items {
            if let syn::Item::Fn(function) = item {
                if function.attrs.iter().any(|attr| {
                    attr.path()
                        .segments
                        .last()
                        .map_or(false, |seg| seg.ident == "event")
                }) {
                    events.push(function.sig.ident.clone());
                }
            }
        }
    }

    let expanded = quote! {
        #client_obj.register_events(vec![#(#events()),*]);
    };

    TokenStream::from(expanded)
}

/// Usage: `register_all!(client => ["src/file.rs", "src/file2.rs"]);`
/// Where `client` is the client object and the array is the list of files to search for events, commands, and slash commands.
/// If the array is empty, it will recursively search for files in the `src` directory.
#[proc_macro]
pub fn register_all(input: TokenStream) -> TokenStream {
    let RegisterCmd {
        client_obj,
        file_array,
    } = parse_macro_input!(input as RegisterCmd);

    let paths: Vec<String> = if !file_array.elems.is_empty() {
        file_array
            .elems
            .into_iter()
            .map(|elem| {
                if let syn::Expr::Lit(lit) = elem {
                    if let syn::Lit::Str(lit_str) = lit.lit {
                        return lit_str.value();
                    }
                }
                panic!("Invalid expression provided");
            })
            .collect()
    } else {
        let mut paths = Vec::new();
        for entry in walkdir::WalkDir::new("src") {
            let entry = entry.unwrap();
            if entry.file_type().is_file() {
                paths.push(entry.path().to_string_lossy().into_owned());
            }
        }
        paths
    };

    let mut events = Vec::new();
    let mut commands = Vec::new();
    let mut slash_commands = Vec::new();

    for path in &paths {
        let items = syn::parse_file(&std::fs::read_to_string(&path).unwrap())
            .unwrap()
            .items;

        for item in items {
            if let syn::Item::Fn(function) = item {
                if function.attrs.iter().any(|attr| {
                    attr.path()
                        .segments
                        .last()
                        .map_or(false, |seg| seg.ident == "event")
                }) {
                    events.push(function.sig.ident.clone());
                } else if function.attrs.iter().any(|attr| {
                    attr.path()
                        .segments
                        .last()
                        .map_or(false, |seg| seg.ident == "command")
                }) {
                    commands.push(function.sig.ident.clone());
                } else if function.attrs.iter().any(|attr| {
                    attr.path()
                        .segments
                        .last()
                        .map_or(false, |seg| seg.ident == "slash")
                }) {
                    slash_commands.push(function.sig.ident.clone());
                }
            }
        }
    }

    let expanded = quote! {
        #client_obj.register_events(vec![#(#events()),*]);
        #client_obj.register_commands(vec![#(#commands()),*]);
        #client_obj.register_slash_commands(vec![#(#slash_commands()),*]).await;
    };

    TokenStream::from(expanded)
}

struct RegisterCmd {
    client_obj: Ident,
    file_array: ExprArray,
}

impl Parse for RegisterCmd {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let client_obj = input.parse()?;
        input.parse::<Token![=>]>()?;
        let file_array = input.parse()?;

        Ok(Self {
            client_obj,
            file_array,
        })
    }
}
