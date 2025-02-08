use dessins_lib::{chapter_1::polygon::ParamsInner, update, Model, NP};
use nannou::prelude::*;

fn model(app: &App) -> Model {
    ParamsInner {
        k: 3,
        r: NP as f32 * 0.45,
        ad: 0_f32,
    }
    .model(app)
}

fn main() {
    nannou::app(model).update(update).run();
}
