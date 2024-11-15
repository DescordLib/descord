use darling::ast::NestedMeta;
use darling::{Error, FromMeta};
use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::visit_mut::{self, VisitMut};
use syn::{parse_macro_input, ExprArray, ExprMethodCall, Ident, ItemFn, Token};

struct ReturnVisitor;
impl VisitMut for ReturnVisitor {
    fn visit_expr_return_mut(&mut self, i: &mut syn::ExprReturn) {
        *i = syn::parse_quote! {
            return Ok(())
        };
    }
}

struct UnwrapVisitor {
    has_unwrap: bool,
}

impl VisitMut for UnwrapVisitor {
    fn visit_expr_method_call_mut(&mut self, i: &mut ExprMethodCall) {
        if i.method == "unwrap" {
            self.has_unwrap = true;
        }
    }
}

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
            /// Returns true if only one of the events is turned on.
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

            pub fn get(&mut self, fn_name: &str, param_name: &proc_macro2::TokenStream) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
                match () {
                    $(
                        _ if self.$event_name
                            || fn_name.to_lowercase() == stringify!($event_name).to_lowercase()
                        => {
                            self.$event_name = true;

                            (
                                quote! { descord::internals::HandlerValue::$arg_type(#param_name) },
                                quote! { descord::Event::$event_ty },
                            )
                        },
                    )*

                    _ => panic!("Enable one of {:?} event\nYou can enable 'one' event like `#[event(event_name)]` or just naming the function same as the event name.", self.all_events()),
                }
            }

            /// Make sure to call `get(...)` before this in case
            /// the event type was not set from the proc macro
            /// and is to be inferred from function name.
            pub fn get_arg_name(&self) -> String {
                match () {
                    $(
                        _ if self.$event_name =>
                            stringify!($arg_type).to_string(),
                    )*

                    _ => panic!("no event is set, call `get(...)` before calling this function"),
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

event_handler_args![
//  event switch       => event type         : event data type
    ready              => Ready              : ReadyData,
    message_create     => MessageCreate      : Message,
    message_delete     => MessageDelete      : Message,
    message_delete_raw => MessageDeleteRaw   : DeletedMessage,
    message_update     => MessageUpdate      : Message,
    reaction_add       => MessageReactionAdd : Reaction,
    guild_create       => GuildCreate        : GuildCreate,
    interaction_create => InteractionCreate  : Interaction,
    member_join        => GuildMemberAdd     : Member,
    member_leave       => GuildMemberRemove  : MemberLeave,
];

#[derive(Debug, FromMeta)]
struct ComponentArgs {
    id: String,
}

#[proc_macro_attribute]
pub fn component(args: TokenStream, input: TokenStream) -> TokenStream {
    let function = parse_macro_input!(input as ItemFn);
    let function_vis = function.vis;
    let function_name = &function.sig.ident;
    let function_body = function.block;

    let attr_args = match NestedMeta::parse_meta_list(args.into()) {
        Ok(v) => v,
        Err(e) => {
            return TokenStream::from(Error::from(e).write_errors());
        }
    };

    let component_args: ComponentArgs = match ComponentArgs::from_list(&attr_args) {
        Ok(v) => v,
        Err(e) => {
            return TokenStream::from(Error::from(e).write_errors());
        }
    };

    let custom_id = component_args.id.to_string();

    let param_name = match function.sig.inputs.first().unwrap() {
        syn::FnArg::Typed(x) => match *x.pat {
            syn::Pat::Ident(ref ident) => quote! { #ident },
            syn::Pat::Wild(ref ident) => quote! { #ident },
            _ => panic!("unknown parameter name"),
        },
        _ => panic!("self???"),
    };

    let let_stmt = quote! {
        let #param_name = data;
    };

    let expanded = quote! {
        #function_vis fn #function_name() -> descord::internals::ComponentHandler {
            use descord::prelude::*;

            fn f(
                data: Interaction,
            ) -> std::pin::Pin<Box<dyn std::future::Future<Output = DescordResult> + Send + 'static>> {
                Box::pin(async move {
                    #let_stmt
                    #function_body
                    Ok(())
                })
            }

            internals::ComponentHandler {
                id: #custom_id.to_string(),
                handler_fn: f,
            }
        }
    };

    TokenStream::from(expanded)
}

#[derive(Debug, FromMeta)]
struct CommandArgs {
    #[darling(default)]
    name: Option<String>,
    #[darling(default)]
    prefix: Option<String>,
    #[darling(multiple)]
    permissions: Vec<String>,
    #[darling(default)]
    description: Option<String>,
}

#[proc_macro_attribute]
pub fn event(args: TokenStream, input: TokenStream) -> TokenStream {
    let function = parse_macro_input!(input as ItemFn);
    let function_vis = function.vis;
    let function_name = &function.sig.ident;
    let function_params = &function.sig.inputs;
    let mut function_body = function.block;
    let mut visitor = ReturnVisitor;

    visit_mut::visit_block_mut(&mut visitor, &mut function_body);
    let mut visitor = UnwrapVisitor { has_unwrap: false };
    visit_mut::visit_block_mut(&mut visitor, &mut function_body);

    if function.sig.asyncness.is_none() {
        panic!("Function marked with `#[descord::event(...)]` should be async");
    }

    let attr_args = match NestedMeta::parse_meta_list(args.into()) {
        Ok(v) => v,
        Err(e) => {
            return TokenStream::from(Error::from(e).write_errors());
        }
    };

    let mut handler_args: EventHandlerArgs = match EventHandlerArgs::from_list(&attr_args) {
        Ok(v) => v,
        Err(e) => {
            return TokenStream::from(Error::from(e).write_errors());
        }
    };

    // if visitor.has_unwrap {
        // println!("Warning: Function '{}' uses .unwrap(). Consider using ? operator if unwrapping a Result for proper error handling", function_name);
    // }

    if function.sig.inputs.len() != 1 {
        panic!("Expected only one parameter");
    }

    // let (name, event_ty) = handler_args.get(&function_name.to_string(), &param_name);
    let (name, event_ty) = match function_params.first().unwrap() {
        syn::FnArg::Typed(param) => {
            let param_name = match *param.pat {
                syn::Pat::Ident(ref ident) => quote! { #ident },
                syn::Pat::Wild(ref ident) => quote! { #ident },
                _ => panic!("unknown parameter name"),
            };

            let ret = handler_args.get(&function_name.to_string(), &param_name);

            let ty = (*param.ty).clone();
            let syn::Type::Path(ref path) = ty else {
                panic!("Expected a path found something else");
            };

            let syn::PathSegment { ident, .. } = path.path.segments.last().unwrap();
            let ident = ident.to_string();
            let expected_type_name = handler_args.get_arg_name();

            if expected_type_name != ident {
                panic!("Expected parameter type `{expected_type_name}` but found `{ident}`");
            }

            ret
        }

        _ => panic!("uh"),
    };

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
            ) -> std::pin::Pin<Box<dyn std::future::Future<Output = DescordResult> + Send + 'static>> {
                Box::pin(async move {
                    #let_stmt
                    #function_body
                    Ok(())
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

    let description = command_args
        .description
        .unwrap_or("No description provided".to_string());

    let custom_prefix = command_args.prefix.is_some();
    let new_name = format!(
        "{}{}",
        command_args.prefix.as_ref().unwrap_or(&String::new()),
        command_args
            .name
            .unwrap_or_else(|| function.sig.ident.to_string())
    );

    let function_name = &function.sig.ident;
    let permissions = command_args.permissions;
    let mut function_body = function.block;
    let mut visitor = ReturnVisitor;

    visit_mut::visit_block_mut(&mut visitor, &mut function_body);
    let mut visitor = UnwrapVisitor { has_unwrap: false };
    visit_mut::visit_block_mut(&mut visitor, &mut function_body);

    // if visitor.has_unwrap {
        // println!("Warning: Function '{}' uses .unwrap(). Consider using ? operator if unwrapping a Result for proper error handling", function_name);
    // }

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
    let mut stmts = vec![];
    let mut optional_params = vec![];

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

        let (name, ty, optional) = match path
            .path
            .segments
            .last()
            .unwrap()
            .ident
            .to_string()
            .as_str()
        {
            "Option" => {
                let mut inner_type = String::new();
                match &path.path.segments.last().unwrap().arguments {
                    syn::PathArguments::AngleBracketed(angle_bracketed_data) => {
                        for arg in &angle_bracketed_data.args {
                            if let syn::GenericArgument::Type(syn::Type::Path(type_path)) = arg {
                                inner_type =
                                    type_path.path.segments.last().unwrap().ident.to_string();
                            }
                        }
                    }
                    _ => panic!("Expected AngleBracketed PathArguments"),
                }
                match inner_type.as_str() {
                    "String" => (type_path!(StringOption, name), type_name!(String), true),

                    "isize" => (type_path!(IntOption, name), type_name!(Int), true),
                    "bool" => (type_path!(BoolOption, name), type_name!(Bool), true),
                    "Channel" => (type_path!(ChannelOption, name), type_name!(Channel), true),
                    "User" => (type_path!(UserOption, name), type_name!(User), true),
                    _ => panic!("Unsupported type"),
                }
            }
            "String" => (type_path!(String, name), type_name!(String), false),
            "isize" => (type_path!(Int, name), type_name!(Int), false),
            "bool" => (type_path!(Bool, name), type_name!(Bool), false),
            "Channel" => (type_path!(Channel, name), type_name!(Channel), false),
            "User" => (type_path!(User, name), type_name!(User), false),
            "Args" => {
                stop = true; // will stop the loop from running again
                (type_path!(Args, name), type_name!(Args), false)
            }

            _ => panic!("Unsupported type"),
        };

        optional_params.push(optional);
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
            ) -> std::pin::Pin<Box<dyn std::future::Future<Output = DescordResult> + Send + 'static>> {
                Box::pin(async move {
                    #let_stmts
                    drop(args);
                    #function_body
                    Ok(())
                })
            }

            internals::Command {
                name: String::from(#new_name),
                fn_sig: vec![#(#param_types),*],
                description: String::from(#description),
                handler_fn: f,
                custom_prefix: #custom_prefix,
                optional_params: vec![#(#optional_params),*],
                permissions: vec![#(#permissions.to_string()),*],
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
    #[darling(multiple)]
    permissions: Vec<String>,
}

#[derive(Debug, FromMeta)]
struct SlashOptionArgs {
    #[darling(default)]
    doc: Option<String>,
    #[darling(default)]
    rename: Option<String>,
    autocomplete: Option<syn::ExprPath>,
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

    let permissions = slash_command_args.permissions;

    let new_name = slash_command_args
        .name
        .unwrap_or_else(|| function.sig.ident.to_string());

    let description = slash_command_args
        .description
        .unwrap_or_else(|| String::from("No description provided"));

    let function_name = &function.sig.ident;
    let mut function_body = function.block;
    let mut visitor = ReturnVisitor;
    visit_mut::visit_block_mut(&mut visitor, &mut function_body);
    let mut visitor = UnwrapVisitor { has_unwrap: false };
    visit_mut::visit_block_mut(&mut visitor, &mut function_body);
    // if visitor.has_unwrap {
        // println!("Warning: Function '{}' uses .unwrap(). Consider using ? operator if unwrapping a Result for proper error handling", function_name);
    // }
    let function_params = &function.sig.inputs;
    let function_vis = function.vis;

    let error =
        || -> ! { panic!("Expected `descord::prelude::Interaction` as the first argument") };
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
    let mut param_autocomplete = vec![];
    let mut param_renames = vec![];
    let mut optional_params = vec![];
    let mut stmts: Vec<proc_macro2::TokenStream> = vec![];

    let stop = false;
    for (idx, param) in function_params.iter().skip(1).enumerate() {
        if stop {
            panic!("`Arg` should be the last parameter");
        }

        let mut param = match param {
            syn::FnArg::Typed(x) => x,
            _ => panic!("`self` is not allowed"),
        }
        .clone();

        let syn::Pat::Ident(name) = &*param.pat else {
            panic!();
        };

        param_names.push(quote! { stringify!(#name).to_string() });

        let attrs: Vec<_> = param
            .attrs
            .drain(..)
            .map(|attr| NestedMeta::Meta(attr.meta))
            .collect();

        let param_attr = match SlashOptionArgs::from_list(&attrs) {
            Ok(v) => v,
            Err(e) => return TokenStream::from(e.write_errors()),
        };

        param_autocomplete.push(if let Some(autocomplete_fn) = param_attr.autocomplete {
            quote! { Some(
                |query: String| Box::pin(async move {
                    #autocomplete_fn(query).await.into_iter().take(25).collect::<Vec<_>>()
                })
            ) }
        } else {
            quote! { None }
        });

        param_renames.push(if let Some(new_name) = param_attr.rename {
            quote! { Some(String::from(#new_name)) }
        } else {
            quote! { None }
        });

        let doc = param_attr
            .doc
            .map(|i| i.trim().to_string())
            .unwrap_or("...".to_string());
        param_descriptions.push(doc);

        let type_ = (*param.ty).clone();
        let syn::Type::Path(ref path) = type_ else {
            panic!("Expected a path found something else");
        };

        let (name, ty, optional) = match path
            .path
            .segments
            .last()
            .unwrap()
            .ident
            .to_string()
            .as_str()
        {
            "Option" => {
                let mut inner_type = String::new();
                match &path.path.segments.last().unwrap().arguments {
                    syn::PathArguments::AngleBracketed(angle_bracketed_data) => {
                        for arg in &angle_bracketed_data.args {
                            if let syn::GenericArgument::Type(syn::Type::Path(type_path)) = arg {
                                inner_type =
                                    type_path.path.segments.last().unwrap().ident.to_string();
                            }
                        }
                    }
                    _ => panic!("Expected AngleBracketed PathArguments"),
                }
                match inner_type.as_str() {
                    "String" => (type_path!(StringOption, name), type_name!(String), true),
                    "isize" => (type_path!(IntOption, name), type_name!(Int), true),
                    "bool" => (type_path!(BoolOption, name), type_name!(Bool), true),
                    "Channel" => (type_path!(ChannelOption, name), type_name!(Channel), true),
                    "User" => (type_path!(UserOption, name), type_name!(User), true),
                    _ => panic!("Unsupported type"),
                }
            }
            "String" => (type_path!(String, name), type_name!(String), false),
            "isize" => (type_path!(Int, name), type_name!(Int), false),
            "bool" => (type_path!(Bool, name), type_name!(Bool), false),
            "Channel" => (type_path!(Channel, name), type_name!(Channel), false),
            "User" => (type_path!(User, name), type_name!(User), false),
            _ => panic!("Unsupported type"),
        };

        optional_params.push(optional);
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
            ) -> std::pin::Pin<Box<dyn std::future::Future<Output = DescordResult> + Send + 'static>> {
                Box::pin(async move {
                    #let_stmts
                    drop(args);
                    #function_body
                    Ok(())
                })
            }

            internals::SlashCommand {
                name: String::from(#new_name),
                description: String::from(#description),
                fn_sig: vec![#(#param_types),*],
                fn_param_names: vec![#(#param_names),*],
                fn_param_descriptions: vec![#(#param_descriptions.to_string()),*],
                fn_param_renames: vec![#(#param_renames),*],
                fn_param_autocomplete: vec![#(#param_autocomplete),*],
                optional_params: vec![#(#optional_params),*],
                permissions: vec![#(#permissions.to_string()),*],
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
        for entry in walkdir::WalkDir::new("src").max_depth(5) {
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
        for entry in walkdir::WalkDir::new("src").max_depth(5) {
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
        for entry in walkdir::WalkDir::new("src").max_depth(5) {
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
        for entry in walkdir::WalkDir::new("src").max_depth(5) {
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
    let mut components = Vec::new();

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
                } else if function.attrs.iter().any(|attr| {
                    attr.path()
                        .segments
                        .last()
                        .map_or(false, |seg| seg.ident == "component")
                }) {
                    components.push(function.sig.ident.clone());
                }
            }
        }
    }

    let expanded = quote! {
        #client_obj.register_events(vec![#(#events()),*]);
        #client_obj.register_commands(vec![#(#commands()),*]);
        #client_obj.register_slash_commands(vec![#(#slash_commands()),*]).await;
        #client_obj.register_component_callbacks(vec![#(#components()),*]);
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
