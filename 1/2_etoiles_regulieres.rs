use common::{
    self,
    chapter_1::star::{self, ParamsInner},
    Model, NP,
};
use nannou::prelude::*;

fn model(app: &App) -> Model {
    let inner = ParamsInner {
        k: 5,
        h: 3,
        r: NP as f32 * 0.45,
        ad: PI / 2.0,
    };

    star::model(app, inner)
}

fn update(_app: &App, model: &mut Model, update: Update) {
    common::update(model, update);
}

fn main() {
    nannou::app(model).update(update).run();
}
