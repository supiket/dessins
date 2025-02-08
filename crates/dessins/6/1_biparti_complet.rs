use dessins_lib::{chapter_6::bipartite::ParamsInner, update, Model, NP};
use nannou::prelude::*;

fn model(app: &App) -> Model {
    ParamsInner {
        n: 10,
        a: pt2((NP as f32) / -2.0, (NP as f32) / -2.0),
        b: pt2((NP as f32) / -2.0, (NP as f32) / 2.0),
        c: pt2((NP as f32) / 2.0, (NP as f32) / -2.0),
        d: pt2((NP as f32) / 2.0, (NP as f32) / 2.0),
    }
    .model(app)
}

fn main() {
    nannou::app(model).update(update).run();
}
