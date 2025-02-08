use dessins_lib::{
    self,
    chapter_1::{
        composition_1::{self, ParamsInner},
        polygon, star,
    },
    Model, NP,
};
use nannou::prelude::*;

fn model(app: &App) -> Model {
    let params = ParamsInner {
        polygon: polygon::ParamsInner {
            k: 5,
            r: NP as f32 * 0.27,
            ad: PI / 2.0,
        },
        star: star::ParamsInner {
            k: 25,
            h: 12,
            r: NP as f32 * 0.22,
            ad: PI / 2.0,
        },
    };

    composition_1::model(app, params)
}

fn update(_app: &App, model: &mut Model, update: Update) {
    dessins_lib::update(model, update);
}

fn main() {
    nannou::app(model).update(update).run();
}
