use common::{
    chapter_5::orbital::{self, CurveSettings, Model, R2Params},
    NP,
};
use nannou::prelude::*;

fn model(app: &App) -> Model {
    let curve = CurveSettings {
        n: 4000,
        t1: 100,
        t2: 1,
        r1: NP as f32 * 0.1,
        k1: 3,
        k2: 2,
        r2_eq: Box::new(r2),
    };

    orbital::model(curve, app)
}

fn r2(params: R2Params) -> f32 {
    NP as f32 * 0.4 * (1.0 - params.i / params.n)
}

fn main() {
    nannou::app(model).update(orbital::update).run();
}
