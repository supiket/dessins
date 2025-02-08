use dessins_lib::{chapter_5::spiral::ParamsInner, update, Model};
use nannou::prelude::*;

fn model(app: &App) -> Model {
    ParamsInner {
        n: 2000,
        t: 40,
        r: 0.8,
        l: 0.1,
        an_factor: 1.0,
    }
    .model(app)
}

fn main() {
    nannou::app(model).update(update).run();
}
