use add_elements::*;
use parse::*;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, Ident, Type};
use types::*;

mod add_elements;
mod parse;
mod types;

#[proc_macro_derive(UiControlledParams, attributes(params, param))]
pub fn derive_design_params(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let _name = input.ident;

    let struct_type = parse_struct_attributes(&input.attrs).unwrap();

    let fields = match input.data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => &fields.named,
            _ => panic!("only named fields are supported"),
        },
        _ => panic!("only structs are supported"),
    };

    let params = fields
        .iter()
        .filter_map(|field| {
            let field_name = field.ident.as_ref().unwrap();
            let field_type = &field.ty;
            parse_field_attributes(&field.attrs, field_name)
                .map(|attrs| match_param_field(field_name, field_type, attrs))
        })
        .collect();

    expand(ExpandParams {
        struct_type,
        params,
    })
}

fn match_param_field(
    field_name: &Ident,
    field_type: &Type,
    attrs: FieldAttributes,
) -> proc_macro2::TokenStream {
    let label = attrs
        .label
        .clone()
        .unwrap_or_else(|| field_name.to_string());

    match field_type {
        Type::Path(type_path) => {
            let type_name = type_path.path.segments.last().unwrap().ident.to_string();
            match type_name.as_str() {
                "usize" | "u32" => {
                    if let Some(range_expr) = attrs.range.as_ref() {
                        let range = parse_range(range_expr);
                        add_numeric_element(label, field_name, range)
                    } else {
                        panic!("fields missing for {}: {}", label, type_name);
                    }
                }
                "f32" => {
                    if attrs.is_pi {
                        add_float_element_pi(label, field_name)
                    } else if attrs.is_length {
                        add_float_element_length(label, field_name)
                    } else if attrs.is_position {
                        add_float_element_position(label, field_name)
                    } else if let Some(range_expr) = attrs.range.as_ref() {
                        let range = parse_range(range_expr);
                        add_numeric_element(label, field_name, range)
                    } else {
                        panic!("fields missing for {}: {}", label, type_name);
                    }
                }
                "ExpressionF32" => {
                    let expr_with_ctx = attrs.expression.expect("in else block after is_none");

                    let range_expr = attrs.range.as_ref().expect("range missing for expression");
                    let range = parse_range(range_expr);

                    add_expression_f32_element(label, field_name, range, expr_with_ctx)
                }
                "Point2" => add_point2_element(label, field_name),
                "Vec" => add_vec_elements(type_path, &attrs, label, field_name),
                _ => quote! {
                    #type_path::ui_elements(&mut self.#field_name, ui)
                },
            }
        }
        _ => panic!("error handling type"),
    }
}

fn expand(ep: ExpandParams) -> TokenStream {
    let ExpandParams {
        struct_type,
        params,
    } = ep;
    let struct_type_lc = struct_type.clone().to_string().to_lowercase();

    TokenStream::from(quote! {
        pub struct Params {
            pub inner: ParamsInner,
            pub calculate_shapes: Box<dyn Fn(&mut ParamsInner) -> Shapes + Send + Sync>,
            pub ui_elements: UiElements,
        }

        pub type UiElements = Box<dyn Fn(&mut ParamsInner, &mut crate::egui::Ui) -> bool + Send + Sync>;

        impl ParamsInner {
            pub fn model(self, app: &App) -> crate::Model {
                let params = crate::DesignParams::#struct_type(Params {
                    inner: self,
                    calculate_shapes: Box::new(Self::calculate_shapes),
                    ui_elements: Box::new(Self::ui_elements),
                });

                crate::model(params, app)
            }

            pub fn ui_elements(&mut self, ui: &mut crate::egui::Ui) -> bool {
                let mut changed = false;
                #(changed |= #params;)*
                changed
            }
        }

        impl Params {
            pub fn ui_design_type(current_design: &crate::DesignParams, ui: &mut crate::egui::Ui) -> Option<crate::DesignParams> {
                let enabled = match current_design {
                    crate::DesignParams::#struct_type(_) => false,
                    _ => true,
                };
                if ui
                    .add_enabled(enabled, crate::egui::Button::new(#struct_type_lc))
                    .clicked()
                {
                    return Some(crate::DesignParams::#struct_type(Params::default()));
                }
                None
            }
        }
    })
}
