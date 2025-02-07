use common::{
    self,
    chapter_3::{self, ui_a0_factor, ui_n, ParamsInner},
    Model, NP,
};
use nannou::prelude::*;
use nannou_egui::egui::Ui;

fn model(app: &App) -> Model {
    let inner = ParamsInner {
        n: 10,
        a0_fn: Box::new(a0_fn),
        a0_factor: 0.0,
        l0_fn: Box::new(l0_fn),
        p0_fn: Box::new(p0_fn),
        rules_fn: Box::new(rules_fn),
    };

    chapter_3::model(app, inner, Box::new(ui_elements))
}

fn update(_app: &App, model: &mut Model, update: Update) {
    common::update(model, update);
}

fn a0_fn(_n: u32, factor: f32) -> f32 {
    factor
}

fn l0_fn(n: u32) -> f32 {
    NP as f32 / (2.0_f32.sqrt().powf(n as f32)) * 0.96
}

fn p0_fn() -> Point2 {
    pt2(NP as f32 * 0.1, -(NP as f32) * 0.65)
}

fn rules_fn(n: u32) -> Vec<i32> {
    let mut rules = vec![0; n as usize + 1];
    for i in (0..=n as usize).step_by(5) {
        rules[i] = 1;
    }
    for i in (0..n as usize).step_by(5) {
        if i + 1 < rules.len() {
            rules[i + 1] = 1;
        }
    }
    rules
}

fn ui_elements(inner: &mut ParamsInner, ui: &mut Ui) -> bool {
    ui_n(inner, ui) || ui_a0_factor(inner, ui)
}

fn main() {
    nannou::app(model).update(update).run();
}
