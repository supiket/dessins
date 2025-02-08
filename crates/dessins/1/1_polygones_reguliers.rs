use dessins_lib::{
    self,
    chapter_1::polygon::{self, ParamsInner},
    Model, NP,
};
use nannou::prelude::*;

fn model(app: &App) -> Model {
    let inner = ParamsInner {
        k: 3,
        r: NP as f32 * 0.45,
        ad: 0_f32,
    };

    polygon::model(app, inner)
}

fn update(_app: &App, model: &mut Model, update: Update) {
    dessins_lib::update(model, update);
}

fn main() {
    nannou::app(model).update(update).run();
}
