use darling::ast::NestedMeta;
use darling::{Error, FromMeta};
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

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

#[derive(Debug, FromMeta)]
struct CommandArgs {
    #[darling(default)]
    name: Option<String>,
    #[darling(default)]
    prefix: Option<String>,
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

    let error = || panic!("Expected `descord::prelude::MessageData` as the first argument");
    match function_params.first() {
        Some(param) => {
            let param = match param {
                syn::FnArg::Typed(x) => x,
                _ => panic!("`self` is not allowed"),
            };

            match *param.ty {
                syn::Type::Path(ref path) if path.path.is_ident("MessageData") => {}
                _ => error(),
            }
        }

        _ => error(),
    }

    let mut param_types = vec![];
    let mut stmts: Vec<proc_macro2::TokenStream> = vec![];

    for (idx, param) in function_params.iter().skip(1).enumerate() {
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
                data: MessageData,
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
                args: vec![#(#param_types),*],
                handler_fn: f,
                custom_prefix: #custom_prefix,
            }
        }
    };

    TokenStream::from(expanded)
}

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
        vec![String::from("src/main.rs")]
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
