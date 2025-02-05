use common::{
    self,
    chapter_5::orbital::{self, CurveParams, R2Params},
    Model, NP,
};
use nannou::prelude::*;

fn model(app: &App) -> Model<CurveParams> {
    let curve = CurveParams {
        n: 2000,
        t1: 2,
        t2: 100,
        r1: NP as f32 * 0.25,
        k1: 1,
        k2: 1,
        r2_eq: Box::new(r2),
    };

    common::model(Box::new(CurveParams::calculate_shapes), curve, app)
}

fn r2(params: R2Params) -> f32 {
    NP as f32 * 0.2 * (1.0 - params.i / params.n)
}

fn main() {
    nannou::app(model).update(orbital::update).run();
}
