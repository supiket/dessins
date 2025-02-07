use common::{
    self,
    chapter_5::rotating::{self, ParamsInner, SParams},
    Model, NP,
};
use nannou::prelude::*;

fn model(app: &App) -> Model {
    let inner = ParamsInner {
        n: 5000,
        t1: 1.0,
        t2: 200.0,
        r1: NP as f32 * 3.0 / 8.0,
        k1: 1,
        k2: 2,
        r2: NP as f32 / 12.0,
        h1: 1,
        h2: 2,
        s_eq: Box::new(s),
    };

    rotating::model(app, inner)
}

fn update(_app: &App, model: &mut Model, update: Update) {
    common::update(model, update);
}

fn s(params: SParams) -> f32 {
    (18.0 * PI * params.i / params.n).cos() * 0.4 + 0.6
}

fn main() {
    nannou::app(model).update(update).run();
}
