use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn, LitStr};

#[proc_macro_attribute]
pub fn command(args: TokenStream, input: TokenStream) -> TokenStream {
    let function = parse_macro_input!(input as ItemFn);
    let name = parse_macro_input!(args as LitStr);

    let function_name = &function.sig.ident;
    let function_body = &function.block;

    let command_name = name.value(); // Get the string value of the name

    let expanded = quote! {
        async fn #function_name(data: descord::prelude::MessageData) -> Result<(), Box<dyn std::error::Error>> {
            if !data.content.starts_with(&("!".to_string() + #command_name)) {
                return Ok(());
            }

            #function_body

            Ok(())
        }
    };

    TokenStream::from(expanded)
}

struct CommandArgs {
    name: LitStr,
}

impl syn::parse::Parse for CommandArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let name = input.parse()?;
        Ok(CommandArgs { name })
    }
}
