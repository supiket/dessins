use dessins_lib::{
    self,
    chapter_7::{self, ParamsInner},
    Model, NP,
};
use nannou::prelude::*;

fn model(app: &App) -> Model {
    let np = NP as f32;

    let inner = ParamsInner {
        m: 4,
        n: 4,
        k: 5,
        positions: vec![
            pt2(-0.5 * np, -0.5 * np),
            pt2(0.5 * np, -0.5 * np),
            pt2(0.5 * np, 0.5 * np),
            pt2(-0.5 * np, 0.5 * np),
            pt2(-0.5 * np, -0.5 * np),
        ],
        lengths: vec![1.0 / (2.0 + 2.0 * (0.45 * PI).cos()); 4],
        angles: vec![0.0, 0.45 * PI, -0.45 * PI, 0.0],
    };

    chapter_7::model(app, inner)
}

fn update(_app: &App, model: &mut Model, update: Update) {
    dessins_lib::update(model, update);
}

fn main() {
    nannou::app(model).update(update).run();
}
