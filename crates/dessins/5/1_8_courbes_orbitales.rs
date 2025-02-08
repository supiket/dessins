use dessins_lib::{
    chapter_5::orbital::{ParamsInner, R2Params},
    update, Model, NP,
};
use nannou::prelude::*;

fn model(app: &App) -> Model {
    ParamsInner {
        n: 5000,
        t1: 2,
        t2: 250,
        r1: NP as f32 * 0.35,
        k1: 1,
        k2: 2,
        r2_eq: Box::new(r2),
    }
    .model(app)
}

fn r2(params: R2Params) -> f32 {
    NP as f32 * 0.15 * (0.5 + 0.5 * (2.0 * PI * params.i / params.n).cos())
}

fn main() {
    nannou::app(model).update(update).run();
}
