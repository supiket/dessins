use common::{
    self,
    chapter_6::linear_modulo::{self, ParamsInner, YParams},
    Model, NP,
};
use nannou::prelude::*;

fn model(app: &App) -> Model {
    let inner = ParamsInner {
        n: 400,
        m: 200,
        k1: 2.0,
        k2: 2.0,
        h: 9,
        i1_factor: 8,
        y_eq: Box::new(y),
    };

    linear_modulo::model(app, inner)
}

fn update(_app: &App, model: &mut Model, update: Update) {
    common::update(model, update);
}

fn y(params: &YParams) -> f32 {
    NP as f32 * 0.5 * (params.k2 * params.i * PI / params.n).cos()
}

fn main() {
    nannou::app(model).update(update).run();
}
