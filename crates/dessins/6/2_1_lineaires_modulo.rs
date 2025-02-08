use dessins_lib::{
    self,
    chapter_6::linear_modulo::{self, ParamsInner, YParams},
    Model, NP,
};
use nannou::prelude::*;

fn model(app: &App) -> Model {
    let inner = ParamsInner {
        n: 400,
        m: 400,
        k1: 4.0,
        k2: 5.0,
        h: 2,
        i1_factor: 1,
        y_eq: Box::new(y),
    };

    linear_modulo::model(app, inner)
}

fn update(_app: &App, model: &mut Model, update: Update) {
    dessins_lib::update(model, update);
}

fn y(params: &YParams) -> f32 {
    NP as f32 * 0.75 * (params.k2 * params.i * PI / params.n).cos()
}

fn main() {
    nannou::app(model).update(update).run();
}
