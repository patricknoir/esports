#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;
extern crate proc_macro;

use proc_macro::TokenStream;
use syn::DeriveInput;

#[proc_macro]
pub fn say_hello(_item: TokenStream) -> TokenStream {
    "fn hello() { println!(\"Hello!\"); }".parse().unwrap()
}

#[proc_macro_derive(CanGenerateJwt)]
pub fn derive_can_generate_jwt(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    // Used in the quasi-quotation below as `#name`.
    let name = input.ident;

    let expanded = quote! {
        impl core::jwt::CanGenerateJwt for #name {}
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(CanDecodeJwt)]
pub fn derive_can_decode_jwt(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    // Used in the quasi-quotation below as `#name`.
    let name = input.ident;

    let expanded = quote! {
        impl core::jwt::CanDecodeJwt<#name> for String {
            fn decode_jwt(&self, secret: String) -> core::prelude::Result<core::jsonwebtoken::TokenData<Claims>> {
            let key= core::jsonwebtoken::DecodingKey::from_secret(secret.as_bytes());
            core::jsonwebtoken::decode(self, &key, &core::jsonwebtoken::Validation::default())
              .map_err(|e| e.into())
            }
        }
    };

    TokenStream::from(expanded)
}