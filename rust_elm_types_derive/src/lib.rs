#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;
extern crate proc_macro;

use proc_macro::TokenStream;
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
    let field_types = find_field_types(&fields);
    for field in &fields {
        let field_ident = field.ident.as_ref().unwrap().to_string();
        let field_type = field_types.get(&field_ident).unwrap();
        let name = find_name_for_field(field);
        println!("name:: {}, type: {:?}", name, field_type);
    }
    quote!().into()
}

fn lit_to_string(lit: &syn::Lit) -> Option<String> {
    match *lit {
        syn::Lit::Str(ref s) => Some(s.value()),
        _ => None,
    }
}

fn find_field_name(meta_items: &Vec<&syn::NestedMeta>) -> Option<String> {
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
                    return find_field_name(&nested.iter().collect());
                }
            },
            _ => unimplemented!(),
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
                    rename = find_field_name(&nested.iter().collect());
                    // println!("Rename :: {:?}", rename);
                }
            }
            // case #[elm]
            Some(syn::Meta::Word(_)) => {}

            // case #[elm = "name_v1"]
            Some(syn::Meta::NameValue(syn::MetaNameValue { .. })) => {}
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
        syn::PathArguments::Parenthesized(ref _paren) => {}
        syn::PathArguments::None => {}
    };
}
