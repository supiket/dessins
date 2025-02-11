use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::Parse, parse_macro_input, Attribute, Data, DeriveInput, ExprRange, Fields, Ident, Type,
};

#[derive(Default)]
struct FieldAttributes {
    label: Option<String>,
    is_length: bool,
    is_position: bool,
    is_pi: bool,
    expression: Option<ExpressionWithContext>,
    range: Option<ExprRange>,
}

#[derive(Default)]
struct ExpressionWithContext {
    expression: String,
    context_vars: Vec<String>,
}

#[proc_macro_derive(UiControlledParams, attributes(params, param))]
pub fn derive_design_params(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let _name = input.ident;

    let struct_param_type = parse_struct_attributes(&input.attrs);
    let struct_param_type_lowercase_string = struct_param_type
        .as_ref()
        .map(|ty| ty.clone().to_string().to_lowercase())
        .unwrap_or("".to_string());

    let fields = match input.data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => &fields.named,
            _ => panic!("Only named fields are supported"),
        },
        _ => panic!("Only structs are supported"),
    };

    let mut param_impls = vec![];

    fields.iter().for_each(|field| {
        let field_name = field.ident.as_ref().unwrap();
        let field_type = &field.ty;
        let attrs = parse_field_attributes(&field.attrs, field_name);

        if let Some(attrs) = attrs {
            param_impls.push(match_param_field(field_name, field_type, attrs))
        }
    });

    let expanded = quote! {
        pub struct Params {
            pub inner: ParamsInner,
            pub calculate_shapes: Box<dyn Fn(&mut ParamsInner) -> Shapes + Send + Sync>,
            pub ui_elements: UiElements,
        }

        pub type UiElements = Box<dyn Fn(&mut ParamsInner, &mut crate::egui::Ui) -> bool + Send + Sync>;

        impl ParamsInner {
            pub fn model(self, app: &App) -> crate::Model {
                let params = crate::DesignParams::#struct_param_type(Params {
                    inner: self,
                    calculate_shapes: Box::new(Self::calculate_shapes),
                    ui_elements: Box::new(Self::ui_elements),
                });

                crate::model(params, app)
            }

            pub fn ui_elements(&mut self, ui: &mut crate::egui::Ui) -> bool {
                let mut changed = false;
                #(changed |= #param_impls;)*
                changed
            }
        }

        impl Params {
            pub fn ui_design_type(current_design: &crate::DesignParams, ui: &mut crate::egui::Ui) -> Option<crate::DesignParams> {
                let enabled = match current_design {
                    crate::DesignParams::#struct_param_type(_) => false,
                    _ => true,
                };
                if ui
                    .add_enabled(enabled, crate::egui::Button::new(#struct_param_type_lowercase_string))
                    .clicked()
                {
                    return Some(crate::DesignParams::#struct_param_type(Params::default()));
                }
                None
            }
        }
    };

    TokenStream::from(expanded)
}

fn match_param_field(
    field_name: &Ident,
    field_type: &Type,
    attrs: FieldAttributes,
) -> proc_macro2::TokenStream {
    let label = attrs.label.unwrap_or_else(|| field_name.to_string());

    match field_type {
        Type::Path(type_path) => {
            let type_name = type_path.path.segments.last().unwrap().ident.to_string();
            match type_name.as_str() {
                "usize" | "u32" => {
                    if let Some(range_expr) = attrs.range.as_ref() {
                        let start = &range_expr.start;
                        let end = &range_expr.end;
                        let limits = match range_expr.limits {
                            syn::RangeLimits::HalfOpen(_) => quote! { .. },
                            syn::RangeLimits::Closed(_) => quote! { ..= },
                        };
                        quote! {
                            crate::ui::add_numeric_slider(ui, #label, &mut self.#field_name, #start #limits #end)
                        }
                    } else {
                        panic!("fields missing for {}: {}", label, type_name);
                    }
                }
                "f32" => {
                    if attrs.is_pi {
                        quote! {
                            crate::ui::add_float_slider_pi(ui, #label, &mut self.#field_name)
                        }
                    } else if attrs.is_length {
                        quote! {
                            crate::ui::add_float_slider_length(ui, #label, &mut self.#field_name)
                        }
                    } else if attrs.is_position {
                        quote! {
                            crate::ui::add_float_slider_position(ui, #label, &mut self.#field_name)
                        }
                    } else if let Some(range_expr) = attrs.range.as_ref() {
                        let start = &range_expr.start;
                        let end = &range_expr.end;
                        let limits = match range_expr.limits {
                            syn::RangeLimits::HalfOpen(_) => quote! { .. },
                            syn::RangeLimits::Closed(_) => quote! { ..= },
                        };
                        quote! {
                            crate::ui::add_numeric_slider(ui, #label, &mut self.#field_name, #start #limits #end)
                        }
                    } else {
                        panic!("fields missing for {}: f32", label);
                    }
                }
                "ExpressionF32" => {
                    let ExpressionWithContext {
                        expression,
                        context_vars,
                    } = attrs.expression.expect("in else block after is_none");
                    let Some(range_expr) = attrs.range.as_ref() else {
                        panic!("range missing for expression");
                    };

                    let start = &range_expr.start;
                    let end = &range_expr.end;
                    let limits = match range_expr.limits {
                        syn::RangeLimits::HalfOpen(_) => quote! { .. },
                        syn::RangeLimits::Closed(_) => quote! { ..= },
                    };
                    let context_vars_idents = context_vars
                        .iter()
                        .map(|s| syn::Ident::new(s, proc_macro2::Span::call_site()));

                    quote! {
                        {
                            use evalexpr::ContextWithMutableVariables;

                            let mut ctx = evalexpr::HashMapContext::new();
                            #(ctx.set_value(#context_vars.to_string(), evalexpr::Value::Float(self.#context_vars_idents as f64)).unwrap();)*
                            self.#field_name.ctx = ctx;
                            crate::ui::add_expression_f32_slider(
                                ui,
                                #label,
                                &mut self.#field_name,
                                #expression,
                                #start #limits #end
                            )
                        }
                    }
                }
                "Point2" => {
                    quote! {
                        crate::ui::add_point2_slider(ui, #label, &mut self.#field_name, -0.5..=0.5)
                    }
                }
                "Vec" => {
                    let inner_type = type_path
                        .path
                        .segments
                        .last()
                        .and_then(|seg| {
                            if let syn::PathArguments::AngleBracketed(generic_args) = &seg.arguments
                            {
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
                                |range_expr| {
                                    let start = &range_expr.start;
                                    let end = &range_expr.end;
                                    let limits = match range_expr.limits {
                                        syn::RangeLimits::HalfOpen(_) => quote! { .. },
                                        syn::RangeLimits::Closed(_) => quote! { ..= },
                                    };

                                    quote! { #start #limits #end }
                                },
                            );
                            quote! {
                                crate::ui::add_point2_vector(ui, #label, &mut self.#field_name, #range)
                            }
                        }
                        "f32" => {
                            let range = if attrs.is_pi {
                                quote! { -PI..=PI }
                            } else if attrs.is_length {
                                quote! { 0.0..=(crate::NP as f32) }
                            } else if attrs.is_position {
                                quote! { -(crate::NP as f32)..=(crate::NP as f32) }
                            } else if let Some(range_expr) = attrs.range.clone() {
                                let start = &range_expr.start;
                                let end = &range_expr.end;
                                let limits = match range_expr.limits {
                                    syn::RangeLimits::HalfOpen(_) => quote! { .. },
                                    syn::RangeLimits::Closed(_) => quote! { ..= },
                                };
                                quote! { #start #limits #end }
                            } else {
                                panic!("range is required for {}: Vec<{}>", label, inner_type_name);
                            };
                            quote! {
                                crate::ui::add_numeric_vector_slider(ui, #label, &mut self.#field_name, #range)
                            }
                        }
                        _ => {
                            if let Some(range_expr) = attrs.range.clone() {
                                let start = &range_expr.start;
                                let end = &range_expr.end;
                                let limits = match range_expr.limits {
                                    syn::RangeLimits::HalfOpen(_) => quote! { .. },
                                    syn::RangeLimits::Closed(_) => quote! { ..= },
                                };

                                quote! {
                                    crate::ui::add_numeric_vector_slider(ui, #label, &mut self.#field_name, #start #limits #end)
                                }
                            } else {
                                panic!("range is required for {}: Vec<{}>", label, inner_type_name);
                            }
                        }
                    }
                }
                _ => quote! {
                    #type_path::ui_elements(&mut self.#field_name, ui)
                },
            }
        }
        _ => panic!("error handling type"),
    }
}

fn parse_struct_attributes(attrs: &[Attribute]) -> Option<proc_macro2::TokenStream> {
    for attr in attrs {
        if attr.path().is_ident("params") {
            let tokens = attr
                .parse_args::<proc_macro2::TokenStream>()
                .expect("Failed to parse params attribute");
            return Some(tokens);
        }
    }
    Option::None
}

fn parse_field_attributes(attrs: &[Attribute], field_name: &syn::Ident) -> Option<FieldAttributes> {
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
                        (syn::UnOp::Neg(_), syn::Expr::Lit(_)) => {}, // -literal is fine
                        (syn::UnOp::Neg(_), syn::Expr::Path(path)) if path.path.is_ident("PI") => {}, // -PI is fine
                        _ => panic!("Range bounds must be literals, negative literals, or PI")
                    }
                                    }
                                    syn::Expr::Path(path) if path.path.is_ident("PI") => {} // PI is fine
                                    _ => panic!(
                                        "Range bounds must be literals, negative literals, or PI"
                                    ),
                                }
                            }

                            param.range = Some(range_expr);
                        }
                    }
                    syn::Meta::List(list) if list.path.is_ident("expression") => {
                        let nested_meta = list
                            .parse_args_with(|input: syn::parse::ParseStream| {
                                input.parse_terminated(syn::Meta::parse, syn::Token![,])
                            })
                            .expect("Failed to parse expression attribute");

                        let mut found_default = false;
                        let mut context_vars = Vec::new();
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
                                                context_vars: Vec::new(),
                                            });
                                        }
                                    }
                                }
                                syn::Meta::List(list) if list.path.is_ident("context") => {
                                    let vars = list
                                        .parse_args_with(|input: syn::parse::ParseStream| {
                                            input
                                                .parse_terminated(syn::Ident::parse, syn::Token![,])
                                        })
                                        .expect("Failed to parse context variables");

                                    context_vars =
                                        vars.into_iter().map(|ident| ident.to_string()).collect();
                                }
                                _ => {}
                            }
                        }

                        if !found_default {
                            panic!("expression attribute must include default = \"...\"");
                        }

                        // If we found an expression, update it with context variables
                        if let Some(eq) = expression_info {
                            param.expression = Some(ExpressionWithContext {
                                expression: eq.expression,
                                context_vars,
                            });
                        } else {
                            panic!("expression attribute value must be a string literal");
                        }
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
