use common::{self, chapter_3::DragonParams, Model, NP};
use nannou::prelude::*;
use nannou_egui::egui::Ui;

fn model(app: &App) -> Model<DragonParams> {
    let n = 10;

    let params = DragonParams {
        n,
        a0_fn: Box::new(a0_fn),
        a0_factor: -1.5 * PI,
        l0_fn: Box::new(l0_fn),
        p0_fn: Box::new(p0_fn),
        rules_fn: Box::new(rules_fn),
    };

    common::model(Box::new(DragonParams::calculate_shapes), params, app)
}

fn update(_app: &App, model: &mut Model<DragonParams>, update: Update) {
    common::update(model, update, ui_elements)
}

fn a0_fn(_n: u32, factor: f32) -> f32 {
    factor
}

fn l0_fn(n: u32) -> f32 {
    NP as f32 / (2.0_f32.sqrt().powf(n as f32)) * 0.9
}

fn p0_fn() -> Point2 {
    pt2(-(NP as f32) * 0.25, (NP as f32) / 6.0)
}

fn rules_fn(n: u32) -> Vec<i32> {
    let mut rules = vec![0; n as usize + 1];
    for i in (0..=n as usize).step_by(4) {
        rules[i] = 1;
    }
    rules
}

fn ui_elements(params: &mut DragonParams, ui: &mut Ui) -> bool {
    params.ui_n(ui) || params.ui_a0_factor(ui)
}

fn main() {
    nannou::app(model).update(update).run();
}
