
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;

use quote::{format_ident, quote};
use syn::{ItemEnum, ItemStruct, parse_macro_input};

use crate::type_::Type;

pub fn serialize_impl(input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as Type);
    let ident = item.get_ident();
    let generics = item.get_generics();

    let new_generic = item.get_new_generic_ident("SeraxSer");
    
    let fn_def = match item {
        Type::Struct(item_struct) => serialize_struct(item_struct),
        Type::Enum(item_enum) => serialize_enum(item_enum),
    };

    quote!(
        impl #generics serax::Serialize for #ident #generics {
            fn serialize<#new_generic : serax::serialize::Serializer>(&self, serializer: #new_generic) 
              -> Result<#new_generic::Ok, #new_generic::Err> {
                use serax::serialize::*;
                #fn_def
            }
        }
    ).into()
}

fn serialize_struct(item_struct: ItemStruct) -> TokenStream2 {
    let struct_name = item_struct.ident.to_string();

    match item_struct.fields {
        syn::Fields::Named(fields_named) => {
            let ident = fields_named.named.into_iter().map(|x| x.ident.unwrap());
            let name = ident.clone().map(|x| x.to_string());
            quote!(
                let mut ser = serializer.into_struct_serializer(#struct_name)?;
                #(ser.serialize_field(#name, &self.#ident)?);*;
                ser.end()
            )
        },
        syn::Fields::Unnamed(fields_unnamed) => {
            if fields_unnamed.unnamed.len() == 1 {
                return quote!(
                    serializer.serialize_newtype_struct_contents(#struct_name, &self.0)
                )
            }

            let i = (0..fields_unnamed.unnamed.len()).map(|x| syn::parse_str::<syn::Index>(&x.to_string()).unwrap());
            quote!(
                let mut ser = serializer.into_struct_unnamed_serializer(#struct_name)?;
                #(ser.serialize_unnamed_field(&self.#i)?);*;
                ser.end()
            )
        },
        syn::Fields::Unit => {
            quote!(
                serializer.serialize_unit_struct(#struct_name)
            )
        },
    }
}

fn serialize_enum(item_enum: ItemEnum) -> TokenStream2 {
    let enum_ident = item_enum.ident;
    let enum_name = enum_ident.to_string();

    let mut cases = Vec::with_capacity(item_enum.variants.len());

    for v in item_enum.variants {
        let ident = v.ident.clone();
        let name = v.ident.to_string();
        cases.push(match v.fields {
            syn::Fields::Named(fields_named) => {
                let field_ident1 = fields_named.named.into_iter().map(|x| x.ident.unwrap());
                let field_ident2 = field_ident1.clone();
                let field_name = field_ident1.clone().map(|x| x.to_string());

                quote!(
                    #enum_ident::#ident{#(#field_ident1),*} => {
                        let mut ser = ser.serialize_struct_variant(#name)?;
                        #(ser.serialize_field(#field_name, #field_ident2)?);*;
                        ser.end()
                    }
                )
            },
            syn::Fields::Unnamed(fields_unnamed) => {
                if fields_unnamed.unnamed.len() == 1 {
                    quote!(
                        #enum_ident::#ident(a0) => {
                            ser.serialize_newtype_variant(#name, a0)
                        }
                    )
                } else {
                    let i1 = (0..fields_unnamed.unnamed.len()).map(|x| format_ident!("a{}", x));
                    let i2 = i1.clone();
                    quote!(
                        #enum_ident::#ident(#(#i1),*) => {
                            let mut ser = ser.serialize_tuple_variant(#name)?;
                            #(ser.serialize_element(#i2)?);*;
                            ser.end()
                        }
                    )
                }
            },
            syn::Fields::Unit => {
                quote!(
                    #enum_ident::#ident => {
                        ser.serialize_unit_variant(#name)
                    }
                )
            },
        });
    }

    quote!(
        let mut ser = serializer.into_enum_serializer(#enum_name)?;
        match self {
            #(#cases),*
        }
    )
}