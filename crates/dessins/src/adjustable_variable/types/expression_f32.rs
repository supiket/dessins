use crate::adjustable_variable::{AdjustableVariable, UpdateVariableParams};
use bevy::reflect::Reflect;
use evalexpr::{ContextWithMutableVariables, HashMapContext};
use nannou::prelude::*;
use std::{collections::HashMap, ops::RangeInclusive};

#[derive(Clone, Debug, PartialEq, Reflect)]
pub struct Context(HashMap<String, f32>);

#[derive(Clone, Debug, PartialEq, Reflect)]
pub struct ExpressionF32 {
    expr: String,
    default_expr: String,
    ctx: Context,
    ctx_ext: HashMap<String, ()>,
    value: f32,
    range: RangeInclusive<f32>,
    step: f32,
}

impl ExpressionF32 {
    pub fn new(
        expr: String,
        default_expr: String,
        ctx: Context,
        ctx_ext: HashMap<String, ()>,
        value: f32,
        range: RangeInclusive<f32>,
        step: f32,
    ) -> Self {
        Self {
            expr,
            default_expr,
            ctx,
            ctx_ext,
            value,
            range,
            step,
        }
    }

    pub fn eval_expr(&mut self) -> f32 {
        self.value =
            evalexpr::eval_number_with_context(&self.expr, &Self::evaluatable_ctx(&self.ctx))
                .unwrap_or_else(|_| {
                    self.expr = self.default_expr.clone();
                    evalexpr::eval_number_with_context(
                        &self.expr,
                        &Self::evaluatable_ctx(&self.ctx),
                    )
                    .expect("default expression has to evaluate")
                }) as f32;

        self.value
    }

    pub fn update_expr(&mut self, expr: &str) {
        self.expr = expr.to_string();
    }

    pub fn get_ctx_keys(&self) -> Vec<String> {
        self.ctx.keys().cloned().collect()
    }

    pub fn insert_ctx_entry(&mut self, key: &str, value: f32) {
        self.ctx.insert(key, value);
    }

    pub fn set_ext_ctx(&mut self, key: &str, value: f32) {
        self.ctx.insert(key, value);
        self.ctx_ext.remove(key);
    }

    pub fn get_value(&self) -> f32 {
        self.value
    }

    fn evaluatable_ctx(context: &Context) -> HashMapContext {
        let mut ctx = HashMapContext::new();
        context.iter().for_each(|(k, v)| {
            ctx.set_value(k.to_string(), evalexpr::Value::Float(*v as f64))
                .expect("context must be valid");
        });

        ctx
    }
}

impl ExpressionF32 {
    fn add_textedit_with_label(&mut self, ui: &mut egui::Ui, label: &str) -> bool {
        let mut changed = false;
        ui.label(label);

        let response = egui::TextEdit::singleline(&mut self.expr)
            .desired_width(120.0)
            .show(ui);

        if response.response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
            if self.ctx_ext.is_empty() {
                let ctx = Self::evaluatable_ctx(&self.ctx);
                if let Ok(value) = evalexpr::eval_number_with_context(&self.expr, &ctx) {
                    self.value = value as f32;
                    changed = true;
                }
            } else {
                changed = true;
            }
        }
        changed
    }
}

impl AdjustableVariable for ExpressionF32 {
    fn update(&mut self, params: UpdateVariableParams) -> bool {
        let UpdateVariableParams {
            ui,
            time: _time,
            name,
        } = params;

        self.add_textedit_with_label(ui, &name)
    }
}

impl Context {
    pub fn new(inner: &[(String, f32)]) -> Self {
        Self(inner.iter().cloned().collect())
    }

    fn insert(&mut self, key: &str, value: f32) {
        self.0.insert(key.to_string(), value);
    }
}

impl core::ops::Deref for Context {
    type Target = HashMap<String, f32>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl core::ops::DerefMut for Context {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
