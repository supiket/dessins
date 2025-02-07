use common::{
    self,
    chapter_6::linear_sticks::{self, ParamsInner, RParams},
    Model, NP,
};
use nannou::prelude::*;

fn model(app: &App) -> Model {
    let inner = ParamsInner {
        n: 100,
        m: 6,
        k: 6,
        r1_eq: Box::new(r1),
        r2_eq: Box::new(r2),
    };

    linear_sticks::model(app, inner)
}

fn update(_app: &App, model: &mut Model, update: Update) {
    common::update(model, update);
}

fn r1(params: &RParams) -> f32 {
    NP as f32 / 3.0 * (7.0).powf(params.i - 1.0)
}

fn r2(params: &RParams) -> f32 {
    NP as f32 / 8.0 * (0.9).powf(params.i - 1.0)
}

fn main() {
    nannou::app(model).update(update).run();
}
