use proc_macro::TokenStream;
use quote::quote;
use syn::{parse::Parse, parse_macro_input, Attribute, Data, DeriveInput, Fields, Lit, Type};

#[derive(Default)]
struct FieldAttributes {
    label: Option<String>,
    is_length: bool,
    is_position: bool,
    is_pi: bool,
    is_np: bool,
    range: Option<(Lit, Lit, bool)>,
}

#[proc_macro_derive(UiControlledParams, attributes(param, params))]
pub fn derive_design_params(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let _name = input.ident;

    let fields = match input.data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => &fields.named,
            _ => panic!("Only named fields are supported"),
        },
        _ => panic!("Only structs are supported"),
    };

    let field_impls = fields.iter().map(|field| {
        let field_name = field.ident.as_ref().unwrap();
        let field_type = &field.ty;
        let attrs = parse_field_attributes(&field.attrs, field_name);

        let label = attrs.label.unwrap_or_else(|| field_name.to_string());

        match field_type {
            Type::Path(type_path) => {
                let type_name = type_path.path.segments.last().unwrap().ident.to_string();
                match type_name.as_str() {
                    "usize" | "u32" => {
                        if let Some((min, max, _is_inclusive)) = attrs.range.clone() {
                            quote! {
                                crate::add_number_slider(ui, #label, &mut self.#field_name, #min..=#max)
                            }
                        } else {
                            quote! { false }
                        }
                    },
                    "f32" => {
                        if attrs.is_pi {
                            quote! {
                                crate::add_float_slider_pi(ui, #label, &mut self.#field_name)
                            }
                        } else if attrs.is_length {
                            quote! {
                                crate::add_float_slider_np_length(ui, #label, &mut self.#field_name)
                            }
                        } else if attrs.is_position {
                            quote! {
                                crate::add_float_slider_np_position(ui, #label, &mut self.#field_name)
                            }
                        } else if let Some((min, max, _is_inclusive)) = attrs.range.clone() {
                            quote! {
                                crate::add_float_slider(ui, #label, &mut self.#field_name, #min..=#max)
                            }
                        } else {
                            quote! { false }
                        }
                    },
                    "Point2" => quote! {
                        crate::add_float_slider_np_position(ui, #label, &mut (self.#field_name).x)
                            || crate::add_float_slider_np_position(ui, #label, &mut (self.#field_name).y)
                    },
                    "Box" => quote! { false },
                    _ => quote! { #type_path::ui_elements(&mut self.#field_name, ui) }
                }
            },
            _ => quote! { false },
        }
    });

    let expanded = quote! {
        pub type UiElements = Box<dyn Fn(&mut ParamsInner, &mut Ui) -> bool>;

        impl ParamsInner {
            pub fn ui_elements(&mut self, ui: &mut nannou_egui::egui::Ui) -> bool {
                false #(|| #field_impls)*
            }
        }
    };

    TokenStream::from(expanded)
}

fn parse_field_attributes(attrs: &[Attribute], field_name: &syn::Ident) -> FieldAttributes {
    let mut field_attrs = FieldAttributes::default();

    for attr in attrs {
        if attr.path().is_ident("param") {
            let nested = attr
                .parse_args_with(|input: syn::parse::ParseStream| {
                    input.parse_terminated(syn::Meta::parse, syn::Token![,])
                })
                .unwrap_or_else(|_| syn::punctuated::Punctuated::new());

            for meta in nested {
                match meta {
                    syn::Meta::NameValue(nv) if nv.path.is_ident("label") => {
                        if let syn::Expr::Lit(expr_lit) = nv.value {
                            if let syn::Lit::Str(lit_str) = expr_lit.lit {
                                field_attrs.label = Some(lit_str.value());
                            }
                        }
                    }
                    syn::Meta::Path(path) if path.is_ident("length") => {
                        field_attrs.is_length = true;
                    }
                    syn::Meta::Path(path) if path.is_ident("position") => {
                        field_attrs.is_position = true;
                    }
                    syn::Meta::Path(path) if path.is_ident("pi") => {
                        field_attrs.is_pi = true;
                    }
                    syn::Meta::Path(path) if path.is_ident("np") => {
                        field_attrs.is_np = true;
                    }
                    syn::Meta::List(list) if list.path.is_ident("range") => {
                        if let Ok(range_expr) = syn::parse2::<syn::ExprRange>(list.tokens.clone()) {
                            if let (Some(start), Some(end)) =
                                (range_expr.start.as_ref(), range_expr.end.as_ref())
                            {
                                let min = match &**start {
                                    syn::Expr::Lit(expr_lit) => expr_lit.lit.clone(),
                                    _ => panic!("Range bounds must be literals"),
                                };
                                let max = match &**end {
                                    syn::Expr::Lit(expr_lit) => expr_lit.lit.clone(),
                                    _ => panic!("Range bounds must be literals"),
                                };
                                let is_inclusive =
                                    matches!(range_expr.limits, syn::RangeLimits::Closed(_));
                                field_attrs.range = Some((min, max, is_inclusive));
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    if field_attrs.label.is_none() {
        field_attrs.label = Some(field_name.to_string());
    }

    field_attrs
}
