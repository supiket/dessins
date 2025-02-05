use common::{
    self,
    chapter_1::{
        polygon::{calculate_polygon, PolygonParams},
        star::{calculate_stars, StarParams},
    },
    Model, Segment, Shape, Shapes, NP,
};
use nannou::prelude::*;
use nannou_egui::{self, egui::Ui};

struct Params {
    polygon: PolygonParams,
    star: StarParams,
}

fn model(app: &App) -> Model<Params> {
    let params = Params {
        polygon: PolygonParams {
            k: 5,
            r: NP as f32 * 0.27,
            ad: PI / 2.0,
        },
        star: StarParams {
            k: 25,
            h: 12,
            r: NP as f32 * 0.22,
            ad: PI / 2.0,
        },
    };

    common::model(Box::new(calculate_shapes), params, app)
}

fn update(_app: &App, model: &mut Model<Params>, update: Update) {
    common::update(model, update, ui_elements);
}

fn calculate_shapes(params: &Params) -> Shapes {
    let mut shapes = Shapes::new();
    let mut shape = Shape::new();

    for i in 0..params.polygon.k {
        let polygon_point = calculate_polygon(&params.polygon, i);

        let mut segment = Segment::new();

        for j in 0..params.star.k {
            let star_point = calculate_stars(&params.star, j);
            let point = star_point + polygon_point;
            segment.push(point);
        }

        segment.push(segment[0]);

        shape.push(segment);
    }

    shapes.push(shape);
    shapes
}

fn ui_elements(params: &mut Params, ui: &mut Ui) -> bool {
    params.polygon.ui_elements(ui) || params.star.ui_elements(ui)
}

fn main() {
    nannou::app(model).update(update).run();
}
