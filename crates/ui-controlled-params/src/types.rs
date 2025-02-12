use syn::ExprRange;

#[derive(Default)]
pub(crate) struct FieldAttributes {
    pub(crate) label: Option<String>,
    pub(crate) is_length: bool,
    pub(crate) is_position: bool,
    pub(crate) is_pi: bool,
    pub(crate) expression: Option<ExpressionWithContext>,
    pub(crate) range: Option<ExprRange>,
}

#[derive(Default)]
pub(crate) struct ExpressionWithContext {
    pub(crate) expression: String,
    pub(crate) ctx: ContextVars,
}

#[derive(Default)]
pub(crate) struct ContextVars {
    pub(crate) int: Vec<String>,
    pub(crate) ext: Vec<String>,
}

pub(crate) struct ExpandParams {
    pub(crate) struct_type: proc_macro2::TokenStream,
    pub(crate) params: Vec<proc_macro2::TokenStream>,
}
