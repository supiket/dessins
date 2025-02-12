use crate::types::*;
use quote::quote;
use syn::{parse::Parse, Attribute};

pub(crate) fn parse_struct_attributes(
    attrs: &[Attribute],
) -> anyhow::Result<proc_macro2::TokenStream> {
    let mut tokens = None;
    for attr in attrs {
        if attr.path().is_ident("params") {
            tokens = Some(
                attr.parse_args::<proc_macro2::TokenStream>()
                    .expect("failed to parse params attribute"),
            );
        }
    }
    tokens.ok_or(anyhow::anyhow!("failed to find params attribute"))
}

pub(crate) fn parse_field_attributes(
    attrs: &[Attribute],
    field_name: &syn::Ident,
) -> Option<FieldAttributes> {
    for attr in attrs {
        if attr.path().is_ident("param") {
            let mut param = FieldAttributes::default();
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
                                param.label = Some(lit_str.value());
                            }
                        }
                    }
                    syn::Meta::Path(path) if path.is_ident("length") => {
                        param.is_length = true;
                    }
                    syn::Meta::Path(path) if path.is_ident("position") => {
                        param.is_position = true;
                    }
                    syn::Meta::Path(path) if path.is_ident("pi") => {
                        param.is_pi = true;
                    }
                    syn::Meta::List(list) if list.path.is_ident("range") => {
                        param.range = parse_range_attr(list).ok();
                    }
                    syn::Meta::List(list) if list.path.is_ident("expression") => {
                        param.expression = parse_expression_attr(list).ok();
                    }
                    _ => {}
                }
            }
            if param.label.is_none() {
                param.label = Some(field_name.to_string());
            }
            return Some(param);
        }
    }
    None
}

pub(crate) fn parse_range_attr(list: syn::MetaList) -> anyhow::Result<syn::ExprRange> {
    if let Ok(range_expr) = syn::parse2::<syn::ExprRange>(list.tokens.clone()) {
        // Validate that we have both bounds
        if range_expr.start.is_none() || range_expr.end.is_none() {
            panic!("Range must have both start and end bounds");
        }

        // Validate each bound is either a literal, -literal, PI, or -PI
        for bound in [range_expr.start.as_ref(), range_expr.end.as_ref()]
            .into_iter()
            .flatten()
        {
            match &**bound {
                syn::Expr::Lit(_) => {} // Literal is fine
                syn::Expr::Unary(expr_unary) => {
                    match (&expr_unary.op, &*expr_unary.expr) {
                        (syn::UnOp::Neg(_), syn::Expr::Lit(_)) => {} // -literal is fine
                        (syn::UnOp::Neg(_), syn::Expr::Path(path)) if path.path.is_ident("PI") => {} // -PI is fine
                        _ => panic!("Range bounds must be literals, negative literals, or PI"),
                    }
                }
                syn::Expr::Path(path) if path.path.is_ident("PI") => {} // PI is fine
                _ => panic!("Range bounds must be literals, negative literals, or PI"),
            }
        }
        Ok(range_expr)
    } else {
        Err(anyhow::anyhow!("todo"))
    }
}

pub(crate) fn parse_expression_attr(list: syn::MetaList) -> anyhow::Result<ExpressionWithContext> {
    let nested_meta = list
        .parse_args_with(|input: syn::parse::ParseStream| {
            input.parse_terminated(syn::Meta::parse, syn::Token![,])
        })
        .expect("Failed to parse expression attribute");

    let mut found_default = false;
    let mut ctx: ContextVars = Default::default();
    let mut expression_info = None;

    for meta in nested_meta {
        match meta {
            syn::Meta::NameValue(nv) if nv.path.is_ident("default") => {
                found_default = true;
                if let syn::Expr::Lit(expr_lit) = nv.value {
                    if let syn::Lit::Str(lit_str) = expr_lit.lit {
                        let eq_str = lit_str.value();

                        expression_info = Some(ExpressionWithContext {
                            expression: eq_str,
                            ctx: Default::default(),
                        });
                    }
                }
            }
            syn::Meta::List(list) if list.path.is_ident("context") => {
                ctx = parse_expression_context(list);
            }
            _ => {}
        }
    }

    if !found_default {
        panic!("expression attribute must include default = \"...\"");
    }

    expression_info
        .map(|eq| ExpressionWithContext {
            expression: eq.expression,
            ctx,
        })
        .ok_or(anyhow::anyhow!(
            "expression attribute value must be a string literal"
        ))
}

pub(crate) fn parse_expression_context(list: syn::MetaList) -> ContextVars {
    let (vars, vars_ext) = list
        .parse_args_with(|input: syn::parse::ParseStream| {
            let mut vars = Vec::new();
            let mut vars_ext = Vec::new();

            while !input.is_empty() {
                // Look ahead to check if we have ext(...) pattern
                if input.peek(syn::Ident) && input.peek2(syn::token::Paren) {
                    let ident: syn::Ident = input.parse()?;
                    if ident == "ext" {
                        // Parse ext(...) content
                        let content;
                        syn::parenthesized!(content in input);
                        let ext_vars =
                            content.parse_terminated(syn::Ident::parse, syn::Token![,])?;
                        vars_ext.extend(ext_vars);
                    } else {
                        return Err(input.error("Expected 'ext' identifier"));
                    }
                } else {
                    // Regular variable
                    vars.push(input.parse::<syn::Ident>()?);
                }

                // Handle optional comma
                if !input.is_empty() {
                    input.parse::<syn::Token![,]>()?;
                }
            }

            Ok((vars, vars_ext))
        })
        .expect("Failed to parse context variables");

    ContextVars {
        int: vars.into_iter().map(|ident| ident.to_string()).collect(),
        ext: vars_ext
            .into_iter()
            .map(|ident| ident.to_string())
            .collect(),
    }
}

pub(crate) fn parse_range(range_expr: &syn::ExprRange) -> proc_macro2::TokenStream {
    let start = &range_expr.start;
    let end = &range_expr.end;
    let limits = match range_expr.limits {
        syn::RangeLimits::HalfOpen(_) => quote! { .. },
        syn::RangeLimits::Closed(_) => quote! { ..= },
    };
    quote! {
        #start #limits #end
    }
}
