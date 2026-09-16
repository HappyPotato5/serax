#![doc(hidden)]

mod type_;
mod serialize;
mod deserialize;

use proc_macro::TokenStream;
//use quote::quote;

use serialize::*;
use deserialize::*;

#[proc_macro_derive(Serialize)]
pub fn serialize(input: TokenStream) -> TokenStream {
    serialize_impl(input)
}

#[proc_macro_derive(Deserialize)]
pub fn deserialize(input: TokenStream) -> TokenStream {
    deserialize_impl(input)
}

// #[proc_macro]
// pub fn test_ser(input: TokenStream) -> TokenStream {
//     let out = serialize_impl(input).to_string();

//     let lit = syn::LitStr::new(&out, proc_macro2::Span::call_site());

//     quote!(println!("{}", #lit);).into()
// }

// #[proc_macro]
// pub fn test_deser(input: TokenStream) -> TokenStream {
//     let out = deserialize_impl(input).to_string();

//     let lit = syn::LitStr::new(&out, proc_macro2::Span::call_site());

//     quote!(println!("{}", #lit);).into()
// }