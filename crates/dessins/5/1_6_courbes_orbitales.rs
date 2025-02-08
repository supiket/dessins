use dessins_lib::{
    self,
    chapter_5::orbital::{self, ParamsInner, R2Params},
    Model, NP,
};
use nannou::prelude::*;

fn model(app: &App) -> Model {
    let inner = ParamsInner {
        n: 4000,
        t1: 1,
        t2: 200,
        r1: NP as f32 * 0.3,
        k1: 3,
        k2: 2,
        r2_eq: Box::new(r2),
    };

    orbital::model(app, inner)
}

fn update(_app: &App, model: &mut Model, update: Update) {
    dessins_lib::update(model, update);
}

fn r2(params: R2Params) -> f32 {
    NP as f32 * 0.2 * (1.0 - params.i / params.n)
}

fn main() {
    nannou::app(model).update(update).run();
}
