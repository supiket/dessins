use common::{
    self,
    chapter_4::{self, ParamsInner},
    Model, NP,
};
use nannou::prelude::*;

fn model(app: &App) -> Model {
    let aa = 4.0 * PI / 5.0;
    let ll = NP as f32;

    let a0 = -aa;
    let p0 = pt2((-ll) / 2.0, 0.0);

    let inner = ParamsInner {
        n: 5,
        k: 5,
        ra: 0.35,
        ll,
        aa,
        a0,
        p0,
    };

    chapter_4::model(app, inner)
}

fn update(_app: &App, model: &mut Model, update: Update) {
    common::update(model, update);
}

fn main() {
    nannou::app(model).update(update).run();
}
