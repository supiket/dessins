use common::{
    self,
    chapter_5::rotating::{self, CurveParams, SParams},
    Model, NP,
};
use nannou::prelude::*;

fn model(app: &App) -> Model<CurveParams> {
    let curve = CurveParams {
        n: 2000,
        t1: 0.5,
        t2: 50.0,
        r1: NP as f32 / 7.0,
        k1: 1,
        k2: 2,
        r2: NP as f32 / 4.0,
        h1: 1,
        h2: 2,
        s_eq: Box::new(s),
    };

    common::model(Box::new(CurveParams::calculate_shapes), curve, app)
}

fn s(_params: SParams) -> f32 {
    1.0
}

fn main() {
    nannou::app(model).update(rotating::update).run();
}
