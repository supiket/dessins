use dessins_lib::{chapter_1::star::ParamsInner, update, Model, NP};
use nannou::prelude::*;

fn model(app: &App) -> Model {
    ParamsInner {
        k: 5,
        h: 3,
        r: NP as f32 * 0.45,
        ad: PI / 2.0,
    }
    .model(app)
}

fn main() {
    nannou::app(model).update(update).run();
}
