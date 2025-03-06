use bevy::reflect::Reflect;
use evalexpr::{ContextWithMutableVariables, HashMapContext};
use nannou::prelude::*;
use std::{collections::HashMap, ops::RangeInclusive};

pub type Context = HashMap<String, f32>;

#[derive(Clone, Debug, PartialEq, Reflect)]
pub struct ExpressionF32 {
    pub expr: String,
    pub ctx: Context,
    pub ctx_ext: HashMap<String, ()>,
    pub val: f32,
}

impl ExpressionF32 {
    pub fn new(expr: String, ctx: Context, ctx_ext: HashMap<String, ()>, val: f32) -> Self {
        Self {
            expr,
            ctx,
            ctx_ext,
            val,
        }
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
    pub fn add_with_label(
        &mut self,
        ui: &mut egui::Ui,
        label: &str,
        default: &str,
        range: RangeInclusive<f32>,
    ) -> bool {
        let mut changed = false;
        ui.label(label);

        if ui
            .add(egui::Slider::new(&mut self.val, range).show_value(false))
            .on_hover_text(format!("default: {}", default))
            .changed()
        {
            changed = true;
            self.expr = format!("{}", self.val);
        }

        let response = egui::TextEdit::singleline(&mut self.expr)
            .desired_width(120.0)
            .show(ui);

        if response.response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
            if self.ctx_ext.is_empty() {
                let ctx = Self::evaluatable_ctx(&self.ctx);
                if let Ok(val) = evalexpr::eval_number_with_context(&self.expr, &ctx) {
                    self.val = val as f32;
                    changed = true;
                }
            } else {
                changed = true;
            }
        }
        changed
    }
}
