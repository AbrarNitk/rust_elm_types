#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;
extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2;
use quote::ToTokens;
use std::collections::HashMap;
use syn::DeriveInput;
mod types;

#[proc_macro_derive(Elm, attributes(elm))]
pub fn generate_elm_types(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let fields = match &ast.data {
        syn::Data::Struct(syn::DataStruct { ref fields, .. }) => {
            if fields.iter().any(|field| field.ident.is_none()) {
                panic!("To use #[derive(Elm)] structs should not have unnamed fields")
            }
            fields.iter().cloned().collect()
        }
        _ => panic!("#[derive(Elm)] can only be used with structs"),
    };
    let types = find_field_types(&fields);
    for field in &fields {
        field_validator_for_field(field, &types);
    }
    quote!().into()
}

fn field_validator_for_field(field: &syn::Field, field_types: &HashMap<String, String>) {
    let field_ident = field.ident.as_ref().unwrap().to_string();

    let error = |msg: &str| -> ! {
        panic!(
            "Invalid attribute #[Elm] on field `{}`: {}",
            field_ident, msg,
        );
    };

    let field_type = field_types.get(&field_ident).unwrap();
    for attr in field.attrs.iter() {
        match attr.interpret_meta() {
            Some(syn::Meta::List(syn::MetaList { ref nested, .. })) => {
                for meta_item in nested.iter() {
                    println!("Meta Item :: {:?}  ", meta_item);
                }
            }
            Some(syn::Meta::Word(word)) => {
                println!("Meta Word :: {:?}", word.to_string());
            }
            Some(syn::Meta::NameValue(syn::MetaNameValue { .. })) => {}
            _ => unreachable!(
                "Got something else other than a list of attributes while checking field `{}`",
                field_ident
            ),
        };
    }
}

fn find_field_types(fields: &Vec<syn::Field>) -> HashMap<String, String> {
    let mut types = HashMap::new();
    for field in fields {
        let field_ident = field.ident.as_ref().unwrap().to_string();

        let field_type = match field.ty {
            syn::Type::Path(syn::TypePath { ref path, .. }) => {
                let mut tokens = proc_macro2::TokenStream::new();
                path.to_tokens(&mut tokens);
                // println!("Path :: {:?}", path);
                tokens.to_string()
            }
            syn::Type::Reference(syn::TypeReference { ref elem, .. }) => {
                let mut tokens = proc_macro2::TokenStream::new();
                elem.to_tokens(&mut tokens);
                tokens.to_string()
            }
            _ => panic!(
                "Type `{:?}` of field `{}` not supported",
                field.ty, field_ident
            ),
        };
        // println!("Field name :: {}, field type: {}", field_ident, field_type);
        types.insert(field_ident, field_type);
    }
    types
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
