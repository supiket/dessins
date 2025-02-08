use dessins_lib::{
    chapter_6::linear_modulo::{ParamsInner, YParams},
    update, Model, NP,
};
use nannou::prelude::*;

fn model(app: &App) -> Model {
    ParamsInner {
        n: 400,
        m: 200,
        k1: 2.0,
        k2: 2.0,
        h: 9,
        i1_factor: 8,
        y_eq: Box::new(y),
    }
    .model(app)
}

fn y(params: &YParams) -> f32 {
    NP as f32 * 0.5 * (params.k2 * params.i * PI / params.n).cos()
}

fn main() {
    nannou::app(model).update(update).run();
}
