use dessins_lib::{
    self,
    chapter_5::spiral::{self, ParamsInner},
    Model,
};
use nannou::prelude::*;

fn model(app: &App) -> Model {
    let inner = ParamsInner {
        n: 2000,
        t: 40,
        r: 0.8,
        l: 0.1,
        an_factor: 1.0,
    };

    spiral::model(app, inner)
}

fn update(_app: &App, model: &mut Model, update: Update) {
    dessins_lib::update(model, update);
}

fn main() {
    nannou::app(model).update(update).run();
}
