use crate::{parse::parse_range, types::*};
use quote::quote;
use syn::{Ident, Type};

pub(crate) fn add_numeric_element(
    label: String,
    field_name: &Ident,
    range: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    quote! {
        crate::ui::add_numeric_slider(ui, #label, &mut self.#field_name, #range)
    }
}

pub(crate) fn add_float_element_pi(label: String, field_name: &Ident) -> proc_macro2::TokenStream {
    quote! {
        crate::ui::add_float_slider_pi(ui, #label, &mut self.#field_name)
    }
}

pub(crate) fn add_float_element_length(
    label: String,
    field_name: &Ident,
) -> proc_macro2::TokenStream {
    quote! {
        crate::ui::add_float_slider_length(ui, #label, &mut self.#field_name)
    }
}

pub(crate) fn add_float_element_position(
    label: String,
    field_name: &Ident,
) -> proc_macro2::TokenStream {
    quote! {
        crate::ui::add_float_slider_position(ui, #label, &mut self.#field_name)
    }
}

pub(crate) fn add_expression_f32_element(
    label: String,
    field_name: &Ident,
    range: proc_macro2::TokenStream,
    expr_with_ctx: ExpressionWithContext,
) -> proc_macro2::TokenStream {
    let ExpressionWithContext {
        expression,
        ctx: ContextVars { int, ext },
    } = expr_with_ctx;

    let ctx_int_idents = int
        .iter()
        .map(|s| syn::Ident::new(s, proc_macro2::Span::call_site()));

    quote! {
        {
            use evalexpr::ContextWithMutableVariables;

            let mut ctx = evalexpr::HashMapContext::new();
            let mut ctx_ext = std::collections::HashMap::new();
            #(ctx.set_value(#int.to_string(), evalexpr::Value::Float(self.#ctx_int_idents as f64)).unwrap();)*
             #(
                 if #ext == "pi" {
                     ctx.set_value("pi".to_string(), evalexpr::Value::Float(std::f32::consts::PI as f64)).unwrap();
                 } else {
                     ctx_ext.insert(#ext.to_string(), ());
                 })*
            self.#field_name.ctx = ctx;
            self.#field_name.ctx_ext = ctx_ext;
            crate::ui::add_expression_f32_slider(
                ui,
                #label,
                &mut self.#field_name,
                #expression,
                #range
            )
        }
    }
}

pub(crate) fn add_point2_element(label: String, field_name: &Ident) -> proc_macro2::TokenStream {
    quote! {
        crate::ui::add_point2_slider(ui, #label, &mut self.#field_name, -0.5..=0.5)
    }
}

pub(crate) fn add_vec_elements(
    type_path: &syn::TypePath,
    attrs: &FieldAttributes,
    label: String,
    field_name: &Ident,
) -> proc_macro2::TokenStream {
    let inner_type = type_path
        .path
        .segments
        .last()
        .and_then(|seg| {
            if let syn::PathArguments::AngleBracketed(generic_args) = &seg.arguments {
                generic_args.args.first()
            } else {
                None
            }
        })
        .and_then(|gen_arg| {
            if let syn::GenericArgument::Type(Type::Path(tp)) = gen_arg {
                Some(tp)
            } else {
                None
            }
        })
        .unwrap_or_else(|| panic!("Could not determine Vec inner type"));

    let inner_type_name = inner_type
        .path
        .segments
        .last()
        .map(|seg| seg.ident.to_string())
        .unwrap_or_default();

    match inner_type_name.as_str() {
        "Point2" => {
            let range = attrs.range.clone().map_or_else(
                || quote! { -1.0..=1.0 },
                |range_expr| parse_range(&range_expr),
            );
            add_point2_vector(label, &field_name, range)
        }
        "f32" => {
            let range = if attrs.is_pi {
                quote! { -PI..=PI }
            } else if attrs.is_length {
                quote! { 0.0..=(crate::NP as f32) }
            } else if attrs.is_position {
                quote! { -(crate::NP as f32)..=(crate::NP as f32) }
            } else if let Some(range_expr) = attrs.range.clone() {
                parse_range(&range_expr)
            } else {
                panic!("range is required for {}: Vec<{}>", label, inner_type_name);
            };
            add_numeric_vector(label, &field_name, range)
        }
        _ => {
            if let Some(range_expr) = attrs.range.clone() {
                let range = parse_range(&range_expr);
                add_numeric_vector(label, &field_name, range)
            } else {
                panic!("range is required for {}: Vec<{}>", label, inner_type_name);
            }
        }
    }
}

pub(crate) fn add_point2_vector(
    label: String,
    field_name: &Ident,
    range: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    quote! {
        crate::ui::add_point2_vector(ui, #label, &mut self.#field_name, #range)
    }
}

pub(crate) fn add_numeric_vector(
    label: String,
    field_name: &Ident,
    range: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    quote! {
        crate::ui::add_numeric_vector_slider(ui, #label, &mut self.#field_name, #range)
    }
}
