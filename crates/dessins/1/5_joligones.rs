use dessins_lib::{
    self,
    chapter_1::jolygon::{self, ParamsInner},
    Model, NP,
};
use nannou::prelude::*;

fn model(app: &App) -> Model {
    let inner = ParamsInner {
        k: 200,
        an: 15.0 * PI / 31.0,
        ra: 0.98,
        aa: 0_f32,
        rr: 0.80 * NP as f32,
    };

    jolygon::model(app, inner)
}

fn update(_app: &App, model: &mut Model, update: Update) {
    dessins_lib::update(model, update);
}

fn main() {
    nannou::app(model).update(update).run();
}
