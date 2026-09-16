
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;

use syn::{ItemEnum, ItemStruct, parse_macro_input};
use quote::{format_ident, quote};

use crate::type_::Type;


pub fn deserialize_impl(input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as Type);

    let item_ident = item.get_ident();
    let generics = item.get_generics();
    let new_generic = item.get_new_generic_ident("SeraxDes");

    let fn_def = match item {
        Type::Struct(item_struct) => deserialize_struct(item_struct),
        Type::Enum(item_enum) => deserialize_enum(item_enum),
    };

    quote!(
        impl #generics serax::Deserialize for #item_ident #generics {
            fn deserialize<#new_generic : serax::deserialize::Deserializer>(deserializer: #new_generic) 
              -> Result<Self, #new_generic::Err> {
                use serax::deserialize::*;
                #fn_def
            }
        }
    ).into()
}

fn deserialize_struct(item_struct: ItemStruct) -> TokenStream2 {
    let struct_ident = item_struct.ident;
    let struct_name = struct_ident.to_string();

    match item_struct.fields {
        syn::Fields::Named(fields_named) => {
            let ident1 = fields_named.named.into_iter().map(|x| x.ident.unwrap());
            let ident2 = ident1.clone();
            let name = ident1.clone().map(|x| x.to_string());

            quote!(
                let mut deser = deserializer.into_struct_deserializer(#struct_name)?;
                #(let #ident1 = deser.deserialize_field(#name)?);*;
                Ok(#struct_ident { #(#ident2),* })
            )
        },
        syn::Fields::Unnamed(fields_unnamed) => {
            if fields_unnamed.unnamed.len() == 1 {
                return quote!(
                    Ok(#struct_ident ( deserializer.deserialize_newtype_struct_contents(#struct_name)? ))
                )
            }

            let ident1 = (0..fields_unnamed.unnamed.len()).map(|x| format_ident!("a{}", x));
            let ident2 = ident1.clone();

            quote!(
                let mut deser = deserializer.into_struct_unnamed_deserializer(#struct_name)?;
                #(let #ident1 = deser.deserialize_unnamed_field()?);*;
                Ok(#struct_ident ( #(#ident2),* ))
            )
        },
        syn::Fields::Unit => {
            quote!(
                deserializer.deserialize_unit_struct(#struct_name)?;

                Ok(#struct_ident)
            )
        },
    }
}

fn deserialize_enum(item_enum: ItemEnum) -> TokenStream2 {
    let enum_ident = item_enum.ident;
    let enum_name = enum_ident.to_string();

    let mut cases = Vec::new();
    for v in item_enum.variants {
        let ident = v.ident;
        let variant_name = ident.to_string();

        cases.push(match v.fields {
            syn::Fields::Named(fields_named) => {
                let ident1 = fields_named.named.into_iter().map(|x| x.ident.unwrap());
                let ident2 = ident1.clone();
                let name = ident1.clone().map(|x| x.to_string());
                quote!(
                    #variant_name => {
                        let mut deser = deser.deserialize_struct_variant()?;

                        #(let #ident1 = deser.deserialize_field(#name)?);*;
                        
                        #enum_ident :: #ident { #(#ident2),* }
                    }
                )
            },
            syn::Fields::Unnamed(fields_unnamed) => {
                if fields_unnamed.unnamed.len() == 1 {
                    quote!(
                        #variant_name => {
                            #enum_ident :: #ident (deser.deserialize_newtype_variant_contents()?)
                        }
                    )
                } else {
                    let i1 = (0..fields_unnamed.unnamed.len()).map(|x| format_ident!("a{}", x));
                    let i2 = i1.clone();

                    quote!(
                        #variant_name => {
                            let mut deser = deser.deserialize_tuple_variant()?;

                            #(let #i1 = deser.deserialize_element()?);*;

                            #enum_ident :: #ident ( #(#i2),* )
                        }
                    )
                }
            },
            syn::Fields::Unit => {
                quote!(
                    #variant_name => {
                        deser.deserialize_unit_variant()?;
                        #enum_ident :: #ident
                    }
                )
            },
        });
    }

    quote!(
        let mut deser = deserializer.into_enum_deserializer(#enum_name)?;

        Ok(match deser.deserialize_variant_ident()?.as_str() {
            #(#cases),*,
            _ => panic!("Unexpected variant identifier"),
        })
    )
}