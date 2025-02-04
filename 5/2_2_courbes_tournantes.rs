use common::{
    chapter_5::rotating::{self, CurveSettings, Model, SParams},
    NP,
};
use nannou::prelude::*;

fn model(app: &App) -> Model {
    let curve = CurveSettings {
        n: 3000,
        t1: 1.0,
        t2: 100.0,
        r1: NP as f32 / 3.0,
        k1: 1,
        k2: 2,
        r2: NP as f32 / 7.0,
        h1: 1,
        h2: 1,
        s_eq: Box::new(s),
    };

    rotating::model(curve, app)
}

fn s(_params: SParams) -> f32 {
    1.0
}

fn main() {
    nannou::app(model).update(rotating::update).run();
}
