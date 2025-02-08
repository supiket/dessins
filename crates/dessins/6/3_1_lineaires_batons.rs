use dessins_lib::{
    chapter_6::linear_sticks::{ParamsInner, RParams},
    update, Model, NP,
};
use nannou::prelude::*;

fn model(app: &App) -> Model {
    ParamsInner {
        n: 100,
        m: 1,
        k: 5,
        r1_eq: Box::new(r1),
        r2_eq: Box::new(r2),
    }
    .model(app)
}

fn r1(_params: &RParams) -> f32 {
    NP as f32 / 4.0
}

fn r2(_params: &RParams) -> f32 {
    NP as f32 * 5.0 / 24.0
}

fn main() {
    nannou::app(model).update(update).run();
}
