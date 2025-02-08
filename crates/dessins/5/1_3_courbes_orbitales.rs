use dessins_lib::{
    chapter_5::orbital::{ParamsInner, R2Params},
    update, Model, NP,
};
use nannou::prelude::*;

fn model(app: &App) -> Model {
    ParamsInner {
        n: 6000,
        t1: 1,
        t2: 300,
        r1: NP as f32 * 0.3,
        k1: 1,
        k2: 2,
        r2_eq: Box::new(r2),
    }
    .model(app)
}

fn r2(params: R2Params) -> f32 {
    NP as f32 * 0.2 * (0.5 + 0.5 * (2.0 * params.i * PI / 10.0).cos())
}

fn main() {
    nannou::app(model).update(update).run();
}
