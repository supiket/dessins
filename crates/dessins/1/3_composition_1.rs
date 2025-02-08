use dessins_lib::{
    chapter_1::{composition_1::ParamsInner, polygon, star},
    update, Model, NP,
};
use nannou::prelude::*;

fn model(app: &App) -> Model {
    ParamsInner {
        polygon: polygon::ParamsInner {
            k: 5,
            r: NP as f32 * 0.27,
            ad: PI / 2.0,
        },
        star: star::ParamsInner {
            k: 25,
            h: 12,
            r: NP as f32 * 0.22,
            ad: PI / 2.0,
        },
    }
    .model(app)
}

fn main() {
    nannou::app(model).update(update).run();
}
