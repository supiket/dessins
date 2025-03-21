use crate::{
    adjustable_variable::types::{Context, ExpressionF32},
    shapes::NP,
};
use evalexpr::{ContextWithMutableVariables, HashMapContext};
use nannou::prelude::*;
use std::{f32::consts::PI, ops::RangeInclusive};

pub fn ui_color(ui: &mut egui::Ui) -> Option<Color> {
    let clicked = ui.button("random color").clicked();
    if clicked {
        Some(Color::srgb(random(), random(), random()))
    } else {
        None
    }
}

fn numeric<'a, T: egui::emath::Numeric>(
    value: &'a mut T,
    osc_ctx: &Context,
    range: RangeInclusive<T>,
) -> egui::Slider<'a> {
    let ctx: HashMapContext<evalexpr::DefaultNumericTypes> =
        ExpressionF32::evaluatable_ctx(&osc_ctx);
    egui::Slider::new(value, range)
        .custom_parser(move |str| evalexpr::eval_number_with_context(str, &ctx).ok())
}

pub fn float<'a>(
    value: &'a mut f32,
    osc_ctx: &Context,
    range: RangeInclusive<f32>,
) -> egui::Slider<'a> {
    let mut ctx: HashMapContext<evalexpr::DefaultNumericTypes> =
        ExpressionF32::evaluatable_ctx(&osc_ctx);
    ctx.set_value("pi".to_string(), evalexpr::Value::Float(f64::PI()))
        .expect("setting to value of same type each time");
    egui::Slider::new(value, range)
        .custom_parser(move |str| evalexpr::eval_number_with_context(str, &ctx).ok())
}

fn float_np<'a>(
    value: &'a mut f32,
    osc_ctx: &Context,
    range: RangeInclusive<f32>,
) -> egui::Slider<'a> {
    float(value, osc_ctx, range).suffix(format!("res (={})", NP))
}

fn float_pi<'a>(value: &'a mut f32, osc_ctx: &Context) -> egui::Slider<'a> {
    float(value, osc_ctx, -2.0..=2.0).suffix("π")
}

pub fn add_numeric<T: egui::emath::Numeric>(
    ui: &mut egui::Ui,
    osc_ctx: &Context,
    label: &str,
    value: &mut T,
    range: RangeInclusive<T>,
) -> bool {
    ui.label(label);
    ui.add(numeric(value, osc_ctx, range))
        .on_hover_ui(|ui| osc_hover_ui(ui, osc_ctx))
        .changed()
}

pub fn add_float_np(
    ui: &mut egui::Ui,
    osc_ctx: &Context,
    value: &mut f32,
    range: RangeInclusive<f32>,
) -> bool {
    let mut val = *value / NP as f32;
    let changed = ui
        .add(float_np(&mut val, osc_ctx, range))
        .on_hover_ui(|ui| osc_hover_ui(ui, osc_ctx))
        .changed();
    *value = val * NP as f32;
    changed
}

pub fn add_float_pi(ui: &mut egui::Ui, osc_ctx: &Context, value: &mut f32) -> bool {
    let mut val = *value / PI;
    let changed = ui
        .add(float_pi(&mut val, osc_ctx))
        .on_hover_ui(|ui| osc_hover_ui(ui, osc_ctx))
        .changed();
    *value = val * PI;
    changed
}

pub fn add_float_position(ui: &mut egui::Ui, osc_ctx: &Context, value: &mut f32) -> bool {
    add_float_np(ui, osc_ctx, value, -1.0..=1.0)
}

pub fn add_float_length(ui: &mut egui::Ui, osc_ctx: &Context, value: &mut f32) -> bool {
    add_float_np(ui, osc_ctx, value, 0.0..=1.0)
}

fn osc_hover_ui(ui: &mut egui::Ui, osc_ctx: &Context) {
    ui.label("context");
    egui::Grid::new("context").num_columns(2).show(ui, |ui| {
        let mut osc_vec = osc_ctx.iter().collect::<Vec<_>>();
        osc_vec.sort_by_key(|&(k, _)| k);
        osc_vec.iter().for_each(|&(k, v)| {
            ui.label(k);
            ui.label(format!("{}", v));
            ui.end_row();
        });
    });
}
