use common::{
    chapter_5::orbital::{self, CurveSettings, Model, R2Params},
    NP,
};
use nannou::prelude::*;

fn model(app: &App) -> Model {
    let curve = CurveSettings {
        n: 4000,
        t1: 1,
        t2: 200,
        r1: NP as f32 * 0.2,
        k1: 1,
        k2: 1,
        r2_eq: Box::new(r2),
    };

    orbital::model(curve, app)
}

fn r2(params: R2Params) -> f32 {
    NP as f32 * 0.3 * (0.4 * (2.0 * params.i * PI / params.n * 5.0).cos() + 0.6)
}

fn main() {
    nannou::app(model).update(orbital::update).run();
}
