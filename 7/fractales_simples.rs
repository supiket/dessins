use common::{
    self,
    chapter_7::{self, SimpleFractalParams},
    Model, NP,
};
use nannou::prelude::*;

fn model(app: &App) -> Model<SimpleFractalParams> {
    let np = NP as f32;

    let params = SimpleFractalParams {
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

    common::model(Box::new(SimpleFractalParams::calculate_shapes), params, app)
}

fn main() {
    nannou::app(model).update(chapter_7::update).run();
}
