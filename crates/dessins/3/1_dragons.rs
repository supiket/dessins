use dessins_lib::{chapter_3::ParamsInner, update, Model, NP};
use nannou::prelude::*;

fn model(app: &App) -> Model {
    ParamsInner {
        n: 6,
        a0_fn: Box::new(a0_fn),
        a0_factor: -PI / 4.0,
        l0_fn: Box::new(l0_fn),
        p0_fn: Box::new(p0_fn),
        rules_fn: Box::new(rules_fn),
    }
    .model(app)
}

fn a0_fn(n: u32, factor: f32) -> f32 {
    (n - 2) as f32 * factor
}

fn l0_fn(n: u32) -> f32 {
    NP as f32 / (2.0_f32.sqrt().powf(n as f32))
}

fn p0_fn() -> Point2 {
    pt2(-(NP as f32) / 6.0, -(NP as f32) / 2.5)
}

fn rules_fn(n: u32) -> Vec<i32> {
    vec![0; n as usize + 1]
}

fn main() {
    nannou::app(model).update(update).run();
}
