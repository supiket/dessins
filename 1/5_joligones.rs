use common::{
    self, add_float_slider, add_float_slider_np, add_float_slider_pi, add_number_slider, Model,
    Segment, Shape, Shapes, NP,
};
use nannou::prelude::*;
use nannou_egui::{self, egui::Ui};

struct Params {
    k: u32,  // # segments
    an: f32, // angle of two consecutive segments
    ra: f32, // ratio of the lengths of two consecutive segments
    aa: f32, // angle of the first segment with horizontal
    rr: f32, // length of the first segment
}

fn model(app: &App) -> Model<Params> {
    let params = Params {
        k: 200,
        an: 15.0 * PI / 31.0,
        ra: 0.98,
        aa: 0_f32,
        rr: 0.80 * NP as f32,
    };

    common::model(Box::new(calculate_shapes), params, app)
}

fn update(_app: &App, model: &mut Model<Params>, update: Update) {
    common::update(model, update, ui_elements);
}

fn calculate_shapes(params: &Params) -> Shapes {
    let mut shapes = Shapes::new();
    let mut shape = Shape::new();
    let mut segment = Segment::new();

    let mut current_length = params.rr;
    let mut current_pos = pt2(0.0, 0.0);
    segment.push(current_pos);

    let mut min_x = 0.0;
    let mut max_x = 0.0;
    let mut min_y = 0.0;
    let mut max_y = 0.0;

    for i in 0..params.k {
        let angle = params.aa + i as f32 * params.an;

        let dx = current_length * angle.cos();
        let dy = current_length * angle.sin();
        let d = pt2(dx, dy);
        let point = current_pos + d;

        // update bounds
        min_x = min_x.min(point.x);
        max_x = max_x.max(point.x);
        min_y = min_y.min(point.y);
        max_y = max_y.max(point.y);

        segment.push(point);
        current_pos = point;
        current_length *= params.ra;
    }

    // calculate center offset
    let center_offset_x = (min_x + max_x) / 2.0;
    let center_offset_y = (min_y + max_y) / 2.0;

    // make segments centered
    for point in &mut segment {
        *point -= pt2(center_offset_x, center_offset_y);
    }

    shape.push(segment);
    shapes.push(shape);

    shapes
}

fn ui_elements(params: &mut Params, ui: &mut Ui) -> bool {
    add_number_slider(ui, "k", &mut params.k, 1..=2500)
        || add_float_slider_pi(ui, "an", &mut params.an, -1.0..=1.0)
        || add_float_slider(ui, "ra", &mut params.ra, 0.0..=1.0)
        || add_float_slider_pi(ui, "aa", &mut params.aa, 0.0..=1.0)
        || add_float_slider_np(ui, "rr", &mut params.rr, 0.0..=1.0)
}

fn main() {
    nannou::app(model).update(update).run();
}
