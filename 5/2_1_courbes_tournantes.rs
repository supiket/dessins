use common::{
    self,
    chapter_5::rotating::{self, CurveParams, SParams},
    Model, NP,
};
use nannou::prelude::*;

fn model(app: &App) -> Model<CurveParams> {
    let curve = CurveParams {
        n: 2000,
        t1: 1.0,
        t2: 100.0,
        r1: NP as f32 / 6.0,
        k1: 1,
        k2: 1,
        r2: NP as f32 / 4.0,
        h1: 1,
        h2: 1,
        s_eq: Box::new(s),
    };

    common::model(Box::new(CurveParams::calculate_shapes), curve, app)
}

fn s(params: SParams) -> f32 {
    (4.0 * PI * params.i / params.n).cos() * 0.4 + 0.6
}

fn main() {
    nannou::app(model).update(rotating::update).run();
}
