#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;
extern crate darling;
extern crate proc_macro;

use crate::types::ElmTypes;
use darling::{FromDeriveInput, FromMeta};
use proc_macro::TokenStream;
use std::collections::HashMap;
use syn::DeriveInput;
mod elm_files;
mod types;

#[derive(Default, FromMeta, Debug)]
#[darling(default)]
struct PathArg {
    rename: Option<String>,
    path: String,
}

#[derive(FromDeriveInput, Debug)]
#[darling(attributes(elm), forward_attrs(allow, doc, cfg))]
struct PathArgs {
    ident: syn::Ident,
    attrs: Vec<syn::Attribute>,
    opts: PathArg,
}

#[proc_macro_derive(Elm, attributes(elm))]
pub fn generate_elm_types(input: TokenStream) -> TokenStream {
    //    let input_clone = input.clone();
    let ast = parse_macro_input!(input as DeriveInput);

    let elm_path = std::env::var("ELM_TYPES").ok();

    let attrs = match PathArgs::from_derive_input(&ast) {
        Ok(val) => val.opts,
        Err(err) => {
            if elm_path.is_some() {
                PathArg {
                    rename: None,
                    path: elm_path.unwrap(),
                }
            } else {
                println!(
                    "export ELM_TYPES dir path or pass elm dir path struct opts: {}",
                    &ast.ident
                );
                return err.write_errors().into();
            }
        }
    };

    let fields = match &ast.data {
        syn::Data::Struct(syn::DataStruct { ref fields, .. }) => {
            if fields.iter().any(|field| field.ident.is_none()) {
                panic!("To use #[derive(Elm)] structs should not have unnamed fields")
            }
            fields.iter().cloned().collect()
        }
        _ => panic!("#[derive(Elm)] can only be used with structs"),
    };
    let field_types = find_field_types(&fields);
    let mut elm_fields = HashMap::new();

    for field in &fields {
        let field_ident = field.ident.as_ref().unwrap().to_string();
        let field_type = field_types
            .get(&field_ident)
            .unwrap()
            .iter()
            .map(|x| x.into())
            .collect::<Vec<ElmTypes>>();

        let name = find_name_for_field(field);
        elm_fields.insert(name, field_type);
        // println!("name:: {}, type: {:?}", name, field_type);
    }

    let ident_name = if let Some(name) = &attrs.rename {
        name.to_string()
    } else {
        ast.ident.to_string()
    };

    elm_files::generate_elm(&attrs.path, &ident_name, &mut elm_fields).unwrap();

    quote!().into()
}

fn lit_to_string(lit: &syn::Lit) -> Option<String> {
    match *lit {
        syn::Lit::Str(ref s) => Some(s.value()),
        _ => None,
    }
}

fn find_field_name(field_ident: &str, meta_items: &Vec<&syn::NestedMeta>) -> Option<String> {
    let mut field_name = None;
    for meta_item in meta_items.iter() {
        match meta_item {
            syn::NestedMeta::Meta(ref item) => match item {
                syn::Meta::Word(_ident) => {
                    continue;
                }
                syn::Meta::NameValue(syn::MetaNameValue {
                    ref ident, ref lit, ..
                }) => {
                    if ident == "rename" {
                        field_name = Some(lit_to_string(lit).unwrap());
                    }
                }
                syn::Meta::List(syn::MetaList { ref nested, .. }) => {
                    return find_field_name(field_ident, &nested.iter().collect());
                }
            },
            _ => unimplemented!("This field is unimplemented: {}", field_ident),
        };

        if field_name.is_some() {
            return field_name;
        }
    }
    field_name
}

fn find_name_for_field(field: &syn::Field) -> String {
    let field_ident = field.ident.as_ref().unwrap().to_string();
    let mut rename = None;
    let error = |msg: &str| -> ! {
        panic!(
            "Invalid attribute #[Elm] on field `{}`: {}",
            field_ident, msg,
        );
    };

    for attr in field.attrs.iter() {
        // If a field doesn't have elm as attr then continue
        if attr.path != parse_quote!(elm) {
            continue;
        }

        match attr.interpret_meta() {
            // case #[elm(rename="name_one")]
            Some(syn::Meta::List(syn::MetaList { ref nested, .. })) => {
                if attr.path == parse_quote!(elm) {
                    rename = find_field_name(&field_ident, &nested.iter().collect());
                    // println!("Rename :: {:?}", rename);
                }
            }
            // case #[elm]
            Some(syn::Meta::Word(_)) => {
                error("case `#[elm]` not implemented");
            }

            // case #[elm = "name_v1"]
            Some(syn::Meta::NameValue(syn::MetaNameValue { .. })) => {
                error("case `#[elm = \"foo\"]` not implemented");
            }
            _ => unreachable!(
                "Got something else other than a list of attributes while checking field `{}`",
                field_ident
            ),
        };
    }

    if let Some(name) = rename {
        name
    } else {
        field_ident
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
                    syn::GenericArgument::Type(syn::Type::Array(syn::TypeArray { .. })) => {
                        unimplemented!("syn::Type::Array unimplemented")
                    }
                    syn::GenericArgument::Type(syn::Type::BareFn(syn::TypeBareFn { .. })) => {
                        unimplemented!("syn::Type::BareFn unimplemented")
                    }
                    syn::GenericArgument::Type(syn::Type::Group(syn::TypeGroup { .. })) => {
                        unimplemented!("syn::Type::Group unimplemented")
                    }
                    syn::GenericArgument::Type(syn::Type::ImplTrait(syn::TypeImplTrait {
                        ..
                    })) => unimplemented!("syn::Type::ImplTrait unimplemented"),
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
        syn::PathArguments::Parenthesized(ref _paren) => {
            unimplemented!("syn::PathArguments::Parenthesized unimplemented")
        }
        syn::PathArguments::None => {
            return;
        }
    };
}
