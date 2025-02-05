use common::{
    self,
    chapter_1::polygon::{calculate_polygon, PolygonParams},
    Model, Segment, Shape, Shapes, NP,
};
use nannou::prelude::*;
use nannou_egui::{self, egui::Ui};

fn model(app: &App) -> Model<PolygonParams> {
    let params = PolygonParams {
        k: 3,
        r: NP as f32 * 0.45,
        ad: 0_f32,
    };

    common::model(Box::new(calculate_shapes), params, app)
}

fn update(_app: &App, model: &mut Model<PolygonParams>, update: Update) {
    common::update(model, update, ui_elements);
}

fn calculate_shapes(params: &PolygonParams) -> Shapes {
    let mut shapes = Shapes::new();
    let mut shape = Shape::new();
    let mut segment = Segment::new();

    for i in 0..params.k {
        let point = calculate_polygon(params, i);
        segment.push(point);
    }

    segment.push(segment[0]);

    shape.push(segment);
    shapes.push(shape);
    shapes
}

fn ui_elements(params: &mut PolygonParams, ui: &mut Ui) -> bool {
    params.ui_elements(ui)
}

fn main() {
    nannou::app(model).update(update).run();
}
