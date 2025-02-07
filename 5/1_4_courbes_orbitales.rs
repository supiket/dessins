use common::{
    self,
    chapter_5::orbital::{self, ParamsInner, R2Params},
    Model, NP,
};
use nannou::prelude::*;

fn model(app: &App) -> Model {
    let inner = ParamsInner {
        n: 2000,
        t1: 1,
        t2: 100,
        r1: NP as f32 * 0.27,
        k1: 1,
        k2: 1,
        r2_eq: Box::new(r2),
    };

    orbital::model(app, inner)
}

fn update(_app: &App, model: &mut Model, update: Update) {
    common::update(model, update);
}

fn r2(params: R2Params) -> f32 {
    NP as f32 * 0.23 * (0.5 * (2.0 * params.i * PI / params.n * 3.0).cos() + 0.5)
}

fn main() {
    nannou::app(model).update(update).run();
}
