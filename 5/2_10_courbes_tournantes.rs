use common::{
    self,
    chapter_5::rotating::{self, ParamsInner, SParams},
    Model, NP,
};
use nannou::prelude::*;

fn model(app: &App) -> Model {
    let inner = ParamsInner {
        n: 4000,
        t1: 1.0,
        t2: 100.0,
        r1: NP as f32 / 6.0,
        k1: 1,
        k2: 2,
        r2: NP as f32 / 4.0,
        h1: 1,
        h2: 1,
        s_eq: Box::new(s),
    };

    rotating::model(app, inner)
}

fn update(_app: &App, model: &mut Model, update: Update) {
    common::update(model, update);
}

fn s(params: SParams) -> f32 {
    (6.0 * PI * params.i / params.n).cos() * 0.4 + 0.6
}

fn main() {
    nannou::app(model).update(update).run();
}
