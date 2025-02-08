use dessins_lib::{
    chapter_6::linear_sticks::{ParamsInner, RParams},
    update, Model, NP,
};
use nannou::prelude::*;

fn model(app: &App) -> Model {
    ParamsInner {
        n: 300,
        m: 4,
        k: 3,
        r1_eq: Box::new(r1),
        r2_eq: Box::new(r2),
    }
    .model(app)
}

fn r1(params: &RParams) -> f32 {
    NP as f32 / 3.0 * (0.6).powf(params.i - 1.0)
}

fn r2(params: &RParams) -> f32 {
    NP as f32 / 8.0 * (0.6).powf(params.i - 1.0)
}

fn main() {
    nannou::app(model).update(update).run();
}
