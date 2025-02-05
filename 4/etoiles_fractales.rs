use common::{
    self,
    chapter_4::{self, FractalParams},
    Model, NP,
};
use nannou::prelude::*;

fn model(app: &App) -> Model<FractalParams> {
    let aa = 4.0 * PI / 5.0;
    let ll = NP as f32;

    let a0 = -aa;
    let p0 = pt2((-ll) / 2.0, 0.0);

    let params = FractalParams {
        n: 5,
        k: 5,
        ra: 0.35,
        ll,
        aa,
        a0,
        p0,
    };

    common::model(Box::new(FractalParams::calculate_shapes), params, app)
}

fn main() {
    nannou::app(model).update(chapter_4::update).run();
}
