extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_derive(CommandOptions)]
pub fn command_options(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    // Extract the struct name and fields
    let struct_name = &input.ident;
    let mut field_names = Vec::new();

    if let Data::Struct(data) = input.data {
        if let Fields::Named(fields) = data.fields {
            for field in fields.named {
                let field_name = field.ident.unwrap();
                field_names.push(field_name);
            }
        }
    }

    // Generate code to implement the to_command_args method
    let expanded = quote! {
        impl #struct_name {
            fn to_command_args(&self) -> Vec<String> {
                let mut args = Vec::new();
                #(if let Some(ref val) = self.#field_names {
                    let field_name = stringify!(#field_names);
                    let mut arg = format!("--{} {}", field_name.replace("_", "-"), val);
                    if arg.ends_with(" ") {
                        arg.pop();
                    }
                    args.push(arg);
                })*
                args
            }
        }
    };

    // Return the generated code as a TokenStream
    TokenStream::from(expanded)
}