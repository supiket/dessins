use nannou::{
    color::{rgb, Srgb},
    geom::Point2,
    rand::random,
    Draw,
};
use nannou_egui::egui::{self, Ui};
use std::{f32::consts::PI, ops::RangeInclusive};

pub mod chapter_1;
pub mod chapter_2;
pub mod chapter_3;
pub mod chapter_4;
pub mod chapter_5;
pub mod chapter_6;

pub const NP: usize = 480; // # elementary steps, i.e. resolution
pub const WEIGHT: f32 = 1.0; // point weight

pub fn draw_closed(draw: &Draw, color: Srgb<u8>, points: &[Point2]) {
    draw_exact(draw, color, points);

    // close curve
    let last = points.last().unwrap();
    let first = points.first().unwrap();
    draw.line()
        .start(*last)
        .end(*first)
        .color(color)
        .weight(WEIGHT);
}

pub fn draw_exact(draw: &Draw, color: Srgb<u8>, points: &[Point2]) {
    if points.len() < 2 {
        return;
    }

    for i in 0..points.len() - 1 {
        let start = points[i];
        let end = points[i + 1];

        draw.line()
            .start(start)
            .end(end)
            .color(color)
            .weight(WEIGHT);
    }
}

pub fn ui_color(ui: &mut Ui) -> Option<Srgb<u8>> {
    let clicked = ui.button("random color").clicked();
    if clicked {
        Some(rgb(random(), random(), random()))
    } else {
        None
    }
}

pub fn add_float_slider(
    ui: &mut Ui,
    label: &str,
    value: &mut f32,
    range: RangeInclusive<f32>,
) -> bool {
    let mut recalculate = false;

    ui.label(label);

    recalculate |= ui
        .add(
            egui::Slider::new(&mut *value, range)
                .custom_parser(|str| evalexpr::eval_float(str).ok()),
        )
        .changed();

    recalculate
}

pub fn add_float_slider_np(
    ui: &mut Ui,
    label: &str,
    value: &mut f32,
    range: RangeInclusive<f32>,
) -> bool {
    let mut recalculate = false;

    ui.label(label);
    let mut val = *value / NP as f32;

    recalculate |= ui
        .add(
            egui::Slider::new(&mut val, range)
                .custom_parser(|str| evalexpr::eval_float(str).ok())
                .suffix(format!("np (={})", NP)),
        )
        .changed();

    *value = val * NP as f32;

    recalculate
}

pub fn add_float_slider_pi(
    ui: &mut Ui,
    label: &str,
    value: &mut f32,
    range: RangeInclusive<f32>,
) -> bool {
    let mut recalculate = false;

    ui.label(label);
    let mut val = *value / PI;

    recalculate |= ui
        .add(
            egui::Slider::new(&mut val, range)
                .custom_parser(|str| evalexpr::eval_float(str).ok())
                .suffix("π"),
        )
        .changed();

    *value = val * PI;

    recalculate
}
