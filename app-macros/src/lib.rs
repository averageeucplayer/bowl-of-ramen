use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};
use convert_case::{Case, Casing};

#[proc_macro_derive(AppEvent)]
pub fn derive_event(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let generics = &input.generics;
    let lifetime_generics = generics.lifetimes().collect::<Vec<_>>();
    
    let lifetime_tokens = if !lifetime_generics.is_empty() {
        quote! { <'a> }
    } else {
        quote! {}
    };

    let struct_name = input.ident;

    let event_name = struct_name.to_string().to_case(Case::Kebab);

    let expanded = quote! {
        impl #lifetime_tokens AppEvent for #struct_name #generics {
            fn event_name(&self) -> &'static str {
                #event_name
            }
        }
    };

    TokenStream::from(expanded)
}