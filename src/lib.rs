#![deny(
    clippy::indexing_slicing,
    clippy::integer_arithmetic,
    clippy::unwrap_used,
    clippy::float_arithmetic
)]
#![allow(clippy::too_many_arguments)]

#[proc_macro_derive(InitFromEnvWithPanicIfFailedWithPanicIfFailed)]
pub fn init_from_env_with_panic_if_failed(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_helpers::panic_location::panic_location();
    use convert_case::Casing;
    let ast: syn::DeriveInput =
        syn::parse(input).expect("InitFromEnvWithPanicIfFailed syn::parse(input) failed");
    let ident = &ast.ident;
    let error_ident = syn::Ident::new(&format!("{ident}Error"), ident.span());
    let error_enum_ident = syn::Ident::new(&format!("{ident}ErrorEnum"), ident.span());
    let error_std_env_var_ident = syn::Ident::new(&format!("{ident}StdEnvVar"), ident.span());
    let error_std_env_var_enum_ident =
        syn::Ident::new(&format!("{ident}ErrorStdEnvEnum"), ident.span());
    let error_parse_ident = syn::Ident::new(&format!("{ident}Parse"), ident.span());
    let error_parse_enum_ident = syn::Ident::new(&format!("{ident}ErrorParseEnum"), ident.span());
    let value_suffix = "_value";
    match ast.data {
        syn::Data::Struct(datastruct) => {
            let capacity = datastruct.fields.len();
            let mut generated_init_struct_fields: Vec<proc_macro2::TokenStream> = Vec::with_capacity(capacity);
            let mut generated_functions: Vec<proc_macro2::TokenStream> = Vec::with_capacity(capacity);
            let mut generated_enum_error_std_env_var_variants: Vec<proc_macro2::TokenStream> = Vec::with_capacity(capacity);
            let mut generated_enum_error_parse_variants: Vec<proc_macro2::TokenStream> = Vec::with_capacity(capacity);
            datastruct.fields.into_iter().for_each(|field| {
                match field.ident.clone() {
                    None => panic!("InitFromEnvWithPanicIfFailed field.ident is None"),
                    Some(field_ident) => {
                        let enum_variant_ident_value =
                            syn::Ident::new(&format!("{field_ident}{value_suffix}"), ident.span());
                        generated_init_struct_fields.push(quote::quote! {
                            #field_ident: #enum_variant_ident_value,
                        });
                    }
                }
                ///////////////////
                let enum_variant_ident_value: syn::Ident;
                let env_var_name: syn::Ident;
                match field.ident.clone() {
                    None => panic!("InitFromEnvWithPanicIfFailed field.ident is None"),
                    Some(field_ident) => {
                        enum_variant_ident_value =
                        syn::Ident::new(&format!("{field_ident}{value_suffix}"), ident.span());
                        env_var_name = syn::Ident::new(
                            &format!("{field_ident}")
                                .to_case(convert_case::Case::Snake)
                                .to_uppercase(),
                            ident.span(),
                        );
                    }
                };
                let enum_variant_type: syn::Path;
                let enum_variant_type_as_string: syn::LitStr;
                match field.ty.clone() {
                    //todo: add different types support
                    syn::Type::Path(type_path) => {
                        enum_variant_type = type_path.path.clone();
                        let mut string_handle = String::from("");
                        if type_path.path.segments.len() == 1 {
                            string_handle = format!("{}", type_path.path.segments[0].ident);
                        } else {
                            for seg in type_path.path.segments {
                                string_handle.push_str(&format!("{}:", seg.ident));
                            }
                            if !string_handle.is_empty() {
                                string_handle.pop();
                            }
                        }
                        enum_variant_type_as_string = syn::LitStr::new(&string_handle, ident.span());
                    }
                    _ => panic!("InitFromEnvWithPanicIfFailed field.ty is not a syn::Type::Path!"),
                };
                let enum_variant_ident = match field.ident.clone() {
                    None => panic!("InitFromEnvWithPanicIfFailed field.ident is None"),
                    Some(field_ident) => syn::Ident::new(
                        &format!("{field_ident}").to_case(convert_case::Case::Pascal),
                        ident.span(),
                    ),
                };
                let env_var_name_as_screaming_snake_case_string =
                    syn::LitStr::new(&format!("{env_var_name}"), ident.span());
                generated_functions.push(quote::quote! {
                    let #enum_variant_ident_value: #enum_variant_type;
                    match std::env::var(#env_var_name_as_screaming_snake_case_string) {
                        Err(e) => {
                            return Err(#error_ident {
                                source: #error_enum_ident::#error_std_env_var_ident(#error_std_env_var_enum_ident::#enum_variant_ident {
                                    source: e,
                                    env_var_name: #env_var_name_as_screaming_snake_case_string,
                                }),
                                was_dotenv_enable,
                            });
                        },
                        Ok(string_handle) => {
                            match string_handle.parse::<#enum_variant_type>() {
                                Err(_) => {
                                    return Err(#error_ident {
                                        source: #error_enum_ident::#error_parse_ident(#error_parse_enum_ident::#enum_variant_ident{
                                            env_var_name: #env_var_name_as_screaming_snake_case_string,
                                            expected_env_var_type: #enum_variant_type_as_string,
                                        }),
                                        was_dotenv_enable,
                                    });
                                },
                                Ok(handle) => {
                                    #enum_variant_ident_value = handle;
                                },
                            }
                        },
                    }
                });
                ///////////
                let enum_variant_ident = match field.ident.clone() {
                    None => panic!("InitFromEnvWithPanicIfFailed field.ident is None"),
                    Some(field_ident) => syn::Ident::new(
                        &format!("{field_ident}").to_case(convert_case::Case::Pascal),
                        ident.span(),
                    ),
                };
                generated_enum_error_std_env_var_variants.push(quote::quote! {
                    #enum_variant_ident {
                        source: std::env::VarError,
                        env_var_name: &'static str,
                    },
                });
                ///////////////
                let enum_variant_ident = match field.ident.clone() {
                    None => panic!("InitFromEnvWithPanicIfFailed field.ident is None"),
                    Some(field_ident) => syn::Ident::new(
                        &format!("{field_ident}").to_case(convert_case::Case::Pascal),
                        ident.span(),
                    ),
                };
                generated_enum_error_parse_variants.push(quote::quote! {
                    #enum_variant_ident {
                        env_var_name: &'static str,
                        expected_env_var_type: &'static str,
                    },
                });
            });
            let generated_init_struct_fields_iter = generated_init_struct_fields.iter();
            let generated_functions_iter = generated_functions.iter();
            let generated_enum_error_std_env_var_variants_iter = generated_enum_error_std_env_var_variants.iter();
            let generated_enum_error_parse_variants_iter = generated_enum_error_parse_variants.iter();
                let gen = quote::quote! {
                #[derive(Debug)]
                pub struct #error_ident {
                    pub source: #error_enum_ident,
                    pub was_dotenv_enable: bool,
                }
                #[derive(Debug)]
                pub enum #error_enum_ident {
                    #error_std_env_var_ident(#error_std_env_var_enum_ident),
                    #error_parse_ident(#error_parse_enum_ident),
                }
                #[derive(Debug)]
                pub enum #error_std_env_var_enum_ident {
                    #(#generated_enum_error_std_env_var_variants_iter)*
                }
                #[derive(Debug)]
                pub enum #error_parse_enum_ident {
                    #(#generated_enum_error_parse_variants_iter)*
                }
                impl #ident {
                    pub fn new() -> Result<Self, #error_ident> {
                        let was_dotenv_enable = dotenv::dotenv().is_ok();
                        #(#generated_functions_iter)*
                        Ok(
                            Self {
                                #(#generated_init_struct_fields_iter)*
                            }
                        )
                    }
                }
            };
            // println!("{gen}");
            gen.into()
        },
        _ => panic!("InitFromEnvWithPanicIfFailed only works on Struct"),
    }
}
