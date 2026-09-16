
//! [![Crates.io](https://img.shields.io/badge/Crates.io-latest-blue?style=for-the-badge&logo=rust&logoColor=black&labelColor=BBBBBB)](https://crates.io/crates/serax)
//! [![Docs.rs](https://img.shields.io/badge/Documentation-Serax-blue?style=for-the-badge&logo=Docs.rs&logoColor=black&labelColor=BBBBBB)](https://docs.rs/serax/latest/serax/)
//! [![Github](https://img.shields.io/badge/Github-Serax-blue?style=for-the-badge&logo=github&logoColor=black&labelColor=BBBBBB)](https://github.com/HappyPotato5/serax)
//! [![License](https://img.shields.io/badge/License-Serax/License-blue?style=for-the-badge&logo=readdotcv&logoColor=black&labelColor=BBBBBB)](https://github.com/HappyPotato5/serax/blob/master/LICENSE)
//! 
//! # Serax_macros
//! The crate that implements all Serax' proc-macros.
//! 
//! <div class="warning">
//! You should never add this crate directly to your dependencies.
//! 
//! This crate is only meant to be used by Serax.
//! </div>

mod type_;
mod serialize;
mod deserialize;

use proc_macro::TokenStream;
//use quote::quote;

use serialize::*;
use deserialize::*;

/// Macro that implements `serax::Serialize`.
#[proc_macro_derive(Serialize)]
pub fn serialize(input: TokenStream) -> TokenStream {
    serialize_impl(input)
}

/// Macro that implements `serax::Deserialize`.
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