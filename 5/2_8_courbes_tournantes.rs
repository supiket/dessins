use common::{
    self,
    chapter_5::rotating::{self, CurveParams, SParams},
    Model, NP,
};
use nannou::prelude::*;

fn model(app: &App) -> Model<CurveParams> {
    let curve = CurveParams {
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

    common::model(Box::new(CurveParams::calculate_shapes), curve, app)
}

fn s(params: SParams) -> f32 {
    (18.0 * PI * params.i / params.n).cos() * 0.4 + 0.6
}

fn main() {
    nannou::app(model).update(rotating::update).run();
}
