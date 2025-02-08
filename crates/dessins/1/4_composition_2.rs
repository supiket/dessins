use dessins_lib::{
    chapter_1::{composition_2::ParamsInner, polygon, star},
    update, Model, NP,
};
use nannou::prelude::*;

fn model(app: &App) -> Model {
    ParamsInner {
        n: 32,
        rr: 0.9,
        polygon: polygon::ParamsInner {
            k: 8,
            r: NP as f32 * 0.36,
            ad: 0_f32,
        },
        star: star::ParamsInner {
            k: 16,
            h: 5,
            r: NP as f32 * 0.14,
            ad: 0_f32,
        },
    }
    .model(app)
}

fn main() {
    nannou::app(model).update(update).run();
}
