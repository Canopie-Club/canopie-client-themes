use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, Lit, Meta, MetaNameValue, parse_macro_input};

#[proc_macro_derive(ThemeConfig, attributes(theme))]
pub fn derive_theme_config(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;

    let fields = match input.data {
        Data::Struct(data) => match data.fields {
            Fields::Named(fields) => fields.named,
            _ => panic!("ThemeConfig only supports named fields"),
        },
        _ => panic!("ThemeConfig can only be derived for structs"),
    };

    let mut property_defs = Vec::new();

    for field in fields {
        let field_name = field.ident.unwrap();
        let field_name_str = field_name.to_string();

        let mut data_type = None;
        let mut interface = None;
        let mut width = None;

        for attr in field.attrs {
            if !attr.path().is_ident("theme") {
                continue;
            }

            let meta = attr.parse_meta().expect("Invalid theme attribute");

            match meta {
                Meta::List(list) => {
                    for nested in list.nested {
                        if let syn::NestedMeta::Meta(Meta::NameValue(MetaNameValue {
                            path,
                            value,
                            ..
                        })) = nested
                        {
                            let ident = path.get_ident().unwrap().to_string();
                            let value = match value {
                                syn::Expr::Lit(expr) => match expr.lit {
                                    Lit::Str(s) => s.value(),
                                    _ => panic!("Expected string literal"),
                                },
                                _ => panic!("Expected literal"),
                            };

                            match ident.as_str() {
                                "data_type" => data_type = Some(value),
                                "interface" => interface = Some(value),
                                "width" => width = Some(value),
                                _ => panic!("Unknown theme attribute: {}", ident),
                            }
                        }
                    }
                }
                _ => panic!("Invalid theme attribute format"),
            }
        }

        let data_type = data_type.expect("Missing data_type");
        let interface = interface.expect("Missing interface");
        let width = width.expect("Missing width");

        let optional = matches!(
            &field.ty,
            syn::Type::Path(tp)
                if tp.path.segments.last().unwrap().ident == "Option"
        );

        property_defs.push(quote! {
            PropertySchema {
                name: #field_name_str,
                data_type: DataType::#data_type,
                interface: Interface::#interface,
                width: Width::#width,
                optional: #optional,
            }
        });
    }

    let expanded = quote! {
        impl ThemeSchema for #struct_name {
            fn schema() -> Vec<PropertySchema> {
                vec![
                    #(#property_defs),*
                ]
            }
        }
    };

    TokenStream::from(expanded)
}
