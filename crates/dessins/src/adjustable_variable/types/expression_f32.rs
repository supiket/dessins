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

    pub fn get_value(&self) -> f32 {
        self.value
    }

    pub fn evaluatable_ctx(context: &Context) -> HashMapContext {
        let mut ctx = HashMapContext::new();
        context.iter().for_each(|(k, v)| {
            ctx.set_value(k.to_string(), evalexpr::Value::Float(*v as f64))
                .expect("context must be valid");
        });

        ctx
    }
}

impl ExpressionF32 {
    fn add_textedit_with_label(
        &mut self,
        ui: &mut egui::Ui,
        osc_ctx: &Context,
        label: &str,
    ) -> bool {
        let mut changed = false;
        ui.label(label);

        let response = egui::TextEdit::singleline(&mut self.expr)
            .desired_width(120.0)
            .show(ui);

        let lost_focus = response.response.lost_focus();
        let has_focus = response.response.has_focus();

        let new_expr_input = lost_focus && ui.input(|i| i.key_pressed(egui::Key::Enter));
        let possible_new_osc_ctx = !lost_focus && !has_focus;

        if new_expr_input || possible_new_osc_ctx {
            let mut new_osc_ctx = false;

            osc_ctx.iter().for_each(|(k, v)| {
                if self.ctx.get(k).is_none() {
                    new_osc_ctx = true;
                    self.ctx.insert(k, *v);
                }

                if let Some(self_v) = self.ctx.get(k) {
                    if self_v != v {
                        new_osc_ctx = true;
                        self.ctx.insert(k, *v);
                    }
                }
            });

            if new_expr_input || new_osc_ctx {
                let ctx = Self::evaluatable_ctx(&self.ctx);
                if let Ok(value) = evalexpr::eval_number_with_context(&self.expr, &ctx) {
                    self.value = value as f32;
                    changed = true;
                }
            }
        }

        response.response.on_hover_ui(|ui| {
            ui.label("default");
            ui.style_mut().interaction.selectable_labels = true;
            ui.label(&self.default_expr);
            ui.style_mut().interaction.selectable_labels = false;
            ui.separator();
            ui.label("context");
            egui::Grid::new("context").num_columns(2).show(ui, |ui| {
                self.ctx.0.iter().for_each(|(k, v)| {
                    if !self.ctx_ext.contains_key(k) {
                        ui.label(k);
                        ui.label(format!("{}", v));
                        ui.end_row();
                    }
                });
            });
        });

        changed
    }
}

impl AdjustableVariable for ExpressionF32 {
    fn update(&mut self, params: UpdateVariableParams) -> bool {
        let UpdateVariableParams {
            ui,
            osc_ctx,
            time: _time,
            name,
        } = params;

        self.add_textedit_with_label(ui, osc_ctx, &name)
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
