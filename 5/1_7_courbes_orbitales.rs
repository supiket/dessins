use common::{
    self,
    chapter_5::orbital::{self, ParamsInner, R2Params},
    Model, NP,
};
use nannou::prelude::*;

fn model(app: &App) -> Model {
    let inner = ParamsInner {
        n: 3000,
        t1: 1,
        t2: 150,
        r1: NP as f32 * 0.25,
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
    NP as f32 * 0.25 * (0.5 + 0.5 * (params.i * PI / params.n).cos())
}

fn main() {
    nannou::app(model).update(update).run();
}
