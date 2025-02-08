use dessins_lib::{
    self,
    chapter_6::linear_sticks::{self, ParamsInner, RParams},
    Model, NP,
};
use nannou::prelude::*;

fn model(app: &App) -> Model {
    let inner = ParamsInner {
        n: 300,
        m: 7,
        k: 7,
        r1_eq: Box::new(r1),
        r2_eq: Box::new(r2),
    };

    linear_sticks::model(app, inner)
}

fn update(_app: &App, model: &mut Model, update: Update) {
    dessins_lib::update(model, update);
}

fn r1(params: &RParams) -> f32 {
    NP as f32 / 3.0 * (0.8).powf(params.i - 1.0)
}

fn r2(params: &RParams) -> f32 {
    NP as f32 / 12.0 * (0.8).powf(params.i - 1.0)
}

fn main() {
    nannou::app(model).update(update).run();
}
