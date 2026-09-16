
use quote::format_ident;
use syn::{GenericParam, Generics, Ident, ItemEnum, ItemStruct, parse::Parse};

pub enum Type {
    Struct(ItemStruct),
    Enum(ItemEnum)
}

impl Type {
    pub fn get_generics(&self) -> Generics {
        match self {
            Type::Struct(item_struct) => item_struct.generics.clone(),
            Type::Enum(item_enum) => item_enum.generics.clone(),
        }
    }

    pub fn get_ident(&self) -> Ident {
        match self {
            Type::Struct(item_struct) => &item_struct.ident,
            Type::Enum(item_enum) => &item_enum.ident,
        }.clone()
    }

    pub fn get_new_generic_ident(&self, mut default: &str) -> Ident {
        let generics = &self.get_generics();

        if default.is_empty() {
            default = "S";
        }

        let mut actual = String::new();
        let mut chars = default.chars().cycle();
        actual.push(chars.next().unwrap());

        while generics.params.iter().any(|x| {
            let GenericParam::Type(ty) = x else { return false };

            ty.ident.to_string() == actual
        }) {
            actual.push(chars.next().unwrap());
        }

        format_ident!("{}", actual)
    }
}

impl Parse for Type {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(match input.parse::<ItemStruct>() {
            Ok(ok) => {
                Self::Struct(ok)
            },
            Err(_) => {
                let en: ItemEnum = input.parse()?;
                Self::Enum(en)
            },
        })
    }
}