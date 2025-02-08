use dessins_lib::{
    chapter_5::orbital::{ParamsInner, R2Params},
    update, Model, NP,
};
use nannou::prelude::*;

fn model(app: &App) -> Model {
    ParamsInner {
        n: 2000,
        t1: 1,
        t2: 100,
        r1: NP as f32 * 0.27,
        k1: 1,
        k2: 1,
        r2_eq: Box::new(r2),
    }
    .model(app)
}

fn r2(params: R2Params) -> f32 {
    NP as f32 * 0.23 * (0.5 * (2.0 * params.i * PI / params.n * 3.0).cos() + 0.5)
}

fn main() {
    nannou::app(model).update(update).run();
}
