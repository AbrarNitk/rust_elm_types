#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;
extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2;
use quote::ToTokens;
use std::collections::HashMap;
use std::env::args;
use std::process::id;
use syn::{DeriveInput, PathSegment};

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

fn field_validator_for_field(
    field: &syn::Field,
    field_types: &HashMap<String, Vec<types::RustType>>,
) {
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

fn find_field_types(fields: &Vec<syn::Field>) -> HashMap<String, Vec<types::RustType>> {
    let mut types = HashMap::new();
    for field in fields {
        let mut type_argument: Vec<types::RustType> = vec![];
        let field_ident = field.ident.as_ref().unwrap().to_string();
        find_field_type(&field.ty, &mut type_argument);
        // println!("Field name :: {}, field type: {:?}", field_ident, type_argument);
        types.insert(field_ident, type_argument);
    }
    types
}

fn find_field_type(field_type: &syn::Type, type_arg: &mut Vec<types::RustType>) {
    match field_type {
        syn::Type::Path(syn::TypePath { ref path, .. }) => {
            let path_segment: &syn::PathSegment = path.segments.last().unwrap().into_value();
            type_arg.push((&path_segment.ident).into());
            // println!("Type Ident :: {:?}", path_segment.ident.to_string());
            type_args(&path_segment.arguments, type_arg);
        }
        syn::Type::Reference(syn::TypeReference { ref elem, .. }) => {
            match elem.as_ref() {
                syn::Type::Path(syn::TypePath { ref path, .. }) => {
                    let path_segment: &syn::PathSegment =
                        path.segments.last().unwrap().into_value();
                    type_arg.push((&path_segment.ident).into());
                    // println!("Ref Type Ident :: {:?}", path_segment.ident.to_string());
                    type_args(&path_segment.arguments, type_arg);
                }
                syn::Type::Reference(syn::TypeReference { ref elem, .. }) => {
                    find_field_type(elem.as_ref(), type_arg);
                }
                _ => {}
            };
        }
        _ => panic!(
            "Type `{:?}` of field `{}` not supported",
            "field.ty", "field_ident"
        ),
    };
}

fn type_args(path_args: &syn::PathArguments, type_arg: &mut Vec<types::RustType>) {
    match path_args {
        syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments {
            ref args,
            ..
        }) => {
            for x in args.iter() {
                match x {
                    syn::GenericArgument::Type(syn::Type::Array(syn::TypeArray {
                        ref elem,
                        ..
                    })) => {
                        println!("TypeArray {:?}", elem);
                    }
                    syn::GenericArgument::Type(syn::Type::BareFn(syn::TypeBareFn { .. })) => {}
                    syn::GenericArgument::Type(syn::Type::Group(syn::TypeGroup { .. })) => {}
                    syn::GenericArgument::Type(syn::Type::ImplTrait(syn::TypeImplTrait {
                        ..
                    })) => {}
                    syn::GenericArgument::Type(syn::Type::Path(syn::TypePath {
                        ref path, ..
                    })) => {
                        let path_segment: &syn::PathSegment =
                            path.segments.last().unwrap().into_value();
                        type_arg.push((&path_segment.ident).into());
                        type_args(&path_segment.arguments, type_arg);
                    }
                    _ => {}
                }
            }
        }
        syn::PathArguments::Parenthesized(ref paren) => {}
        syn::PathArguments::None => {}
    };
}
