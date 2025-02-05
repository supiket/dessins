use common::{
    self,
    chapter_1::star::{calculate_stars, StarParams},
    Model, Segment, Shape, Shapes, NP,
};
use nannou::prelude::*;
use nannou_egui::{self, egui::Ui};

fn model(app: &App) -> Model<StarParams> {
    let params = StarParams {
        k: 5,
        h: 3,
        r: NP as f32 * 0.45,
        ad: PI / 2.0,
    };

    common::model(Box::new(calculate_shapes), params, app)
}

fn update(_app: &App, model: &mut Model<StarParams>, update: Update) {
    common::update(model, update, ui_elements);
}

fn calculate_shapes(params: &StarParams) -> Shapes {
    let mut shapes = Shapes::new();
    let mut shape = Shape::new();
    let mut segment = Segment::new();

    for i in 0..params.k {
        let point = calculate_stars(params, i);
        segment.push(point);
    }

    segment.push(segment[0]);

    shape.push(segment);
    shapes.push(shape);
    shapes
}

fn ui_elements(params: &mut StarParams, ui: &mut Ui) -> bool {
    params.ui_elements(ui)
}

fn main() {
    nannou::app(model).update(update).run();
}
