use dessins_lib::{chapter_1::jolygon::ParamsInner, update, Model, NP};
use nannou::prelude::*;

fn model(app: &App) -> Model {
    ParamsInner {
        k: 200,
        an: 15.0 * PI / 31.0,
        ra: 0.98,
        aa: 0_f32,
        rr: 0.80 * NP as f32,
    }
    .model(app)
}

fn main() {
    nannou::app(model).update(update).run();
}
