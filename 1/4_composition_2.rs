use common::{
    self, add_float_slider, add_number_slider,
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
    n: u32,  // # stars
    rr: f32, // reduction coefficient from one star to the next & the distance between the center of the spiral and the center of successive stars
}

fn model(app: &App) -> Model<Params> {
    let params = Params {
        n: 32,
        rr: 0.9,
        polygon: PolygonParams {
            k: 8,
            r: NP as f32 * 0.36,
            ad: 0_f32,
        },
        star: StarParams {
            k: 16,
            h: 5,
            r: NP as f32 * 0.14,
            ad: 0_f32,
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

    for i in 0..params.n {
        let r2 = params.polygon.r * params.rr.powi(i as i32);
        let r3 = params.star.r * params.rr.powi(i as i32);

        let mut polygon_params = params.polygon.clone();
        polygon_params.r = r2;
        let polygon_point = calculate_polygon(&polygon_params, i);

        let mut segment = Segment::new();

        for j in 0..params.star.k {
            let mut star_params = params.star.clone();
            star_params.r = r3;
            let star_point = calculate_stars(&star_params, j);
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
    add_number_slider(ui, "n", &mut params.n, 1..=100)
        || add_float_slider(ui, "rr", &mut params.rr, 0.0..=1.0)
        || params.polygon.ui_elements(ui)
        || params.star.ui_elements(ui)
}

fn main() {
    nannou::app(model).update(update).run();
}
