use common::{
    self,
    chapter_6::bipartite::{self, ParamsInner},
    Model, NP,
};
use nannou::prelude::*;

fn model(app: &App) -> Model {
    let inner = ParamsInner {
        n: 10,
        a: pt2((NP as f32) / -2.0, (NP as f32) / -2.0),
        b: pt2((NP as f32) / -2.0, (NP as f32) / 2.0),
        c: pt2((NP as f32) / 2.0, (NP as f32) / -2.0),
        d: pt2((NP as f32) / 2.0, (NP as f32) / 2.0),
    };

    bipartite::model(app, inner)
}

fn update(_app: &App, model: &mut Model, update: Update) {
    common::update(model, update);
}

fn main() {
    nannou::app(model).update(update).run();
}
