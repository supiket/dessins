use crate::NP;
use evalexpr::HashMapContext;
use nannou::prelude::*;
use std::{collections::HashMap, f32::consts::PI, ops::RangeInclusive};

pub struct ExpressionF32 {
    pub expr: String,
    pub ctx: HashMapContext,
    pub ctx_ext: HashMap<String, ()>,
    pub val: f32,
}

pub fn ui_color(ui: &mut egui::Ui) -> Option<Color> {
    let clicked = ui.button("random color").clicked();
    if clicked {
        Some(Color::srgb(random(), random(), random()))
    } else {
        None
    }
}

pub fn add_numeric<T: egui::emath::Numeric>(
    ui: &mut egui::Ui,
    label: &str,
    value: &mut T,
    range: RangeInclusive<T>,
) -> bool {
    ui.label(label);
    ui.add(
        egui::Slider::new(&mut *value, range).custom_parser(|str| evalexpr::eval_number(str).ok()),
    )
    .changed()
}

pub fn add_float_position(ui: &mut egui::Ui, label: &str, value: &mut f32) -> bool {
    add_float_np(ui, label, value, -0.5..=0.5)
}

pub fn add_float_length(ui: &mut egui::Ui, label: &str, value: &mut f32) -> bool {
    add_float_np(ui, label, value, 0.0..=1.0)
}

fn add_float_np(
    ui: &mut egui::Ui,
    label: &str,
    value: &mut f32,
    range: RangeInclusive<f32>,
) -> bool {
    ui.label(label);
    let mut val = *value / NP as f32;

    let recalculate = ui
        .add(
            egui::Slider::new(&mut val, range)
                .custom_parser(|str| evalexpr::eval_number(str).ok())
                .suffix(format!("res (={})", NP)),
        )
        .changed();

    *value = val * NP as f32;

    recalculate
}

pub fn add_float_pi(ui: &mut egui::Ui, label: &str, value: &mut f32) -> bool {
    ui.label(label);
    let mut val = *value / PI;

    let recalculate = ui
        .add(
            egui::Slider::new(&mut val, -PI..=PI)
                .custom_parser(|str| evalexpr::eval_number(str).ok())
                .suffix("π"),
        )
        .changed();

    *value = val * PI;

    recalculate
}

pub fn add_point2(
    ui: &mut egui::Ui,
    label: &str,
    value: &mut Point2,
    range: RangeInclusive<f32>,
) -> bool {
    let mut changed = false;

    ui.label(label);

    let mut val = value.x / NP as f32;
    changed |= ui
        .add(
            egui::Slider::new(&mut val, range.clone())
                .text("x")
                .custom_parser(|str| evalexpr::eval_number(str).ok())
                .suffix(format!("res (={})", NP)),
        )
        .changed();
    value.x = val * NP as f32;

    let mut val = value.y / NP as f32;
    changed |= ui
        .add(
            egui::Slider::new(&mut val, range.clone())
                .text("y")
                .custom_parser(|str| evalexpr::eval_number(str).ok())
                .suffix(format!("res (={})", NP)),
        )
        .changed();
    value.y = val * NP as f32;

    changed
}

pub fn add_numeric_vector<T: egui::emath::Numeric>(
    ui: &mut egui::Ui,
    label: &str,
    value: &mut [T],
    range: RangeInclusive<T>,
) -> bool {
    let mut changed = false;
    ui.collapsing(label, |ui| {
        for val in value {
            changed |= ui
                .add(
                    egui::Slider::new(&mut *val, range.clone())
                        .custom_parser(|str| evalexpr::eval_number(str).ok()),
                )
                .changed();
        }
    });
    changed
}

pub fn add_point2_vector(
    ui: &mut egui::Ui,
    label: &str,
    value: &mut [Point2],
    range: RangeInclusive<f32>,
) -> bool {
    let mut changed = false;
    ui.collapsing(label, |ui| {
        for (index, val) in value.iter_mut().enumerate() {
            changed |= add_point2(ui, &index.to_string(), val, range.clone());
        }
    });
    changed
}

pub fn add_expression_f32(
    ui: &mut egui::Ui,
    label: &str,
    value: &mut ExpressionF32,
    default: &str,
    range: RangeInclusive<f32>,
) -> bool {
    let mut changed = false;
    ui.label(label);

    if ui
        .add(egui::Slider::new(&mut value.val, range).show_value(false))
        .on_hover_text(format!("default: {}", default))
        .changed()
    {
        changed = true;
        value.expr = format!("{}", value.val);
    }

    let response = egui::TextEdit::singleline(&mut value.expr)
        .desired_width(120.0)
        .show(ui);

    if response.response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
        if value.ctx_ext.is_empty() {
            if let Ok(val) = evalexpr::eval_number_with_context(&value.expr, &value.ctx) {
                value.val = val as f32;
                changed = true;
            }
        } else {
            changed = true;
        }
    }
    changed
}
