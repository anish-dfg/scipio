use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, GenericArgument, PathArguments, Type};

const SUPPORTED_OPTION_TYPES: [&str; 5] = [
    "Option",
    "::core::option::Option",
    "core::option::Option",
    "::std::option::Option",
    "std::option::Option",
];

fn unwrap_type_twice(ty: &Type) -> (std::option::Option<String>, std::option::Option<String>) {
    if let Type::Path(type_path) = ty {
        let raw_first_segment_path = type_path.path.segments.iter().fold(
            if type_path.path.leading_colon.is_some() {
                "::".to_string()
            } else {
                "".to_string()
            },
            |acc, el| acc + &el.ident.to_string() + "::",
        );

        let args = match type_path.path.segments.len() {
            0 => &PathArguments::None,
            1 => &type_path.path.segments[0].arguments,
            2.. => &type_path.path.segments[type_path.path.segments.len() - 1].arguments,
        };

        let first_segment_path =
            raw_first_segment_path[..raw_first_segment_path.len() - 2].to_owned();

        if let PathArguments::AngleBracketed(path_args) = args {
            let inner = &path_args.args[0];
            if let GenericArgument::Type(Type::Path(inner_type_path)) = inner {
                let raw_second_segment_path = inner_type_path.path.segments.iter().fold(
                    if inner_type_path.path.leading_colon.is_some() {
                        "::".to_string()
                    } else {
                        "".to_string()
                    },
                    |acc, el| acc + &el.ident.to_string() + "::",
                );
                let second_segment_path =
                    raw_second_segment_path[..raw_second_segment_path.len() - 2].to_owned();
                return (Some(first_segment_path), Some(second_segment_path));
            }
        } else {
            return (Some(first_segment_path), None);
        };

        (Some(first_segment_path), None)
    } else {
        (None, None)
    }
}

#[proc_macro_derive(ToQueryString)]
pub fn to_query_string_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let fields = match input.data {
        Data::Struct(data_struct) => match data_struct.fields {
            Fields::Named(fields_named) => fields_named.named,
            _ => {
                return TokenStream::from(quote! {
                    compile_error!("ToQueryString can only be derived for structs with named fields.");
                })
            }
        },
        _ => {
            return TokenStream::from(quote! {
                compile_error!("ToQueryString can only be derived for structs.");
            })
        }
    };

    let field_processings = fields.iter().map(|field| {
        let Some(ref ident) = field.ident else {
            return quote! {
                compile_error!("asdf");
            };
        };

        let field_name = ident;
        let field_name_str = ident.to_string();

        match unwrap_type_twice(&field.ty) {
            (Some(outer), Some(ref inner))
                if SUPPORTED_OPTION_TYPES.contains(&&*outer) && inner == "Vec" =>
            {
                quote! {
                    if let Some(ref v) = self.#field_name {
                        for item in v {
                            let qs = format!("{}={}&", #field_name_str, item);
                            query.push_str(&qs);
                        }
                    }
                }
            }
            (Some(outer), _) if SUPPORTED_OPTION_TYPES.contains(&&*outer) => {
                quote! {
                    if let Some(ref item) = self.#field_name {
                        query.push_str(&format!("{}={}&", #field_name_str, item));
                    }
                }
            }
            (Some(ref outer), _) if outer == "Vec" => {
                quote! {
                    for item in &self.#field_name {
                        query.push_str(&format!("{}={}&", #field_name_str, item));
                    }
                }
            }
            (_, _) => {
                quote! {
                    query.push_str(&format!("{}={}&", #field_name_str, self.#field_name));
                }
            }
        }
    });

    let expanded = quote! {
        impl #name {
            pub fn to_query_string(&self) -> String {
                let mut query = String::from("?");
                #(#field_processings)*
                query.pop(); // Remove the last '&'
                query

            }
        }
    };

    TokenStream::from(expanded)
}
