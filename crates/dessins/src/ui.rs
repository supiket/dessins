use crate::shapes::NP;
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

fn numeric<T: egui::emath::Numeric>(value: &mut T, range: RangeInclusive<T>) -> egui::Slider<'_> {
    egui::Slider::new(value, range).custom_parser(|str| evalexpr::eval_number(str).ok())
}

pub fn float(value: &mut f32, range: RangeInclusive<f32>) -> egui::Slider<'_> {
    let mut ctx: HashMapContext<evalexpr::DefaultNumericTypes> = HashMapContext::new();
    ctx.set_value("pi".to_string(), evalexpr::Value::Float(f64::PI()))
        .expect("setting to value of same type each time");
    egui::Slider::new(value, range)
        .custom_parser(move |str| evalexpr::eval_number_with_context(str, &ctx).ok())
}

fn float_np(value: &mut f32, range: RangeInclusive<f32>) -> egui::Slider<'_> {
    float(value, range).suffix(format!("res (={})", NP))
}

fn float_pi(value: &mut f32) -> egui::Slider<'_> {
    float(value, -2.0..=2.0).suffix("π")
}

pub fn add_numeric<T: egui::emath::Numeric>(
    ui: &mut egui::Ui,
    label: &str,
    value: &mut T,
    range: RangeInclusive<T>,
) -> bool {
    ui.label(label);
    ui.add(numeric(value, range)).changed()
}

pub fn add_float_np(ui: &mut egui::Ui, value: &mut f32, range: RangeInclusive<f32>) -> bool {
    let mut val = *value / NP as f32;
    let changed = ui.add(float_np(&mut val, range)).changed();
    *value = val * NP as f32;
    changed
}

pub fn add_float_pi(ui: &mut egui::Ui, value: &mut f32) -> bool {
    let mut val = *value / PI;
    let changed = ui.add(float_pi(&mut val)).changed();
    *value = val * PI;
    changed
}

pub fn add_float_position(ui: &mut egui::Ui, value: &mut f32) -> bool {
    add_float_np(ui, value, -1.0..=1.0)
}

pub fn add_float_length(ui: &mut egui::Ui, value: &mut f32) -> bool {
    add_float_np(ui, value, 0.0..=1.0)
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
        .add(float_np(&mut val, range.clone()).text("x"))
        .changed();
    value.x = val * NP as f32;

    let mut val = value.y / NP as f32;
    changed |= ui
        .add(float_np(&mut val, range.clone()).text("y"))
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
            changed |= ui.add(numeric(val, range.clone())).changed();
        }
    });
    changed
}

pub fn add_float_vector(
    ui: &mut egui::Ui,
    label: &str,
    value: &mut [f32],
    range: RangeInclusive<f32>,
) -> bool {
    let mut changed = false;
    ui.collapsing(label, |ui| {
        for val in value {
            changed |= ui.add(float(val, range.clone())).changed()
        }
    });
    changed
}

pub fn add_length_vector(ui: &mut egui::Ui, label: &str, value: &mut [f32]) -> bool {
    let mut changed = false;
    ui.collapsing(label, |ui| {
        for val in value {
            changed |= add_float_length(ui, val);
        }
    });
    changed
}

pub fn add_position_vector(ui: &mut egui::Ui, label: &str, value: &mut [f32]) -> bool {
    let mut changed = false;
    ui.collapsing(label, |ui| {
        for val in value {
            changed |= add_float_position(ui, val);
        }
    });
    changed
}

pub fn add_angle_vector(ui: &mut egui::Ui, label: &str, value: &mut [f32]) -> bool {
    let mut changed = false;
    ui.collapsing(label, |ui| {
        for val in value {
            changed |= add_float_pi(ui, val);
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
