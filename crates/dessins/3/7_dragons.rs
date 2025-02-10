use dessins_lib::{chapter_3::ParamsInner, update, Model, NP};
use nannou::prelude::*;

fn model(app: &App) -> Model {
    let n = 10;
    let rules = rules_fn(n);
    ParamsInner {
        n,
        a0_fn: Box::new(a0_fn),
        a0_factor: 0.0,
        l0: l0_fn(n),
        l0_fn: Box::new(l0_fn),
        p0_fn: Box::new(p0_fn),
        rules,
    }
    .model(app)
}

fn a0_fn(_n: u32, factor: f32) -> f32 {
    factor
}

fn l0_fn(n: u32) -> f32 {
    NP as f32 / (2.0_f32.sqrt().powf(n as f32)) * 0.9
}

fn p0_fn() -> Point2 {
    pt2(-(NP as f32) * 0.3, -(NP as f32) * 0.4)
}

fn rules_fn(n: u32) -> Vec<i32> {
    let mut rules = vec![0; n as usize + 1];
    for i in (0..=n as usize).step_by(5) {
        rules[i] = 1;
    }
    for i in (0..=n as usize - 2).step_by(5) {
        if i + 2 < rules.len() {
            rules[i + 2] = 1;
        }
    }
    rules
}

fn main() {
    nannou::app(model).update(update).run();
}
