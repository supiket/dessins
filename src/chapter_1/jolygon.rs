use crate::{
    add_float_slider, add_float_slider_np_length, add_float_slider_pi, add_number_slider,
    DesignParams, Model, Segment, Shape, Shapes,
};
use nannou::prelude::*;
use nannou_egui::egui::Ui;

pub struct ParamsInner {
    pub k: u32,  // # segments
    pub an: f32, // angle of two consecutive segments
    pub ra: f32, // ratio of the lengths of two consecutive segments
    pub aa: f32, // angle of the first segment with horizontal
    pub rr: f32, // length of the first segment
}

pub struct Params {
    pub inner: ParamsInner,
    pub calculate_shapes: Box<dyn Fn(&ParamsInner) -> Shapes>,
    pub ui_elements: Box<dyn Fn(&mut ParamsInner, &mut Ui) -> bool>,
}

pub fn model(app: &App, inner: ParamsInner) -> Model {
    let params = DesignParams::Jolygon(Params {
        inner,
        calculate_shapes: Box::new(calculate_shapes),
        ui_elements: Box::new(ui_elements),
    });

    crate::model(params, app)
}

pub fn calculate_shapes(inner: &ParamsInner) -> Shapes {
    let mut shapes = Shapes::new();
    let mut shape = Shape::new();
    let mut segment = Segment::new();

    let mut current_length = inner.rr;
    let mut current_pos = pt2(0.0, 0.0);
    segment.push(current_pos);

    let mut min_x = 0.0;
    let mut max_x = 0.0;
    let mut min_y = 0.0;
    let mut max_y = 0.0;

    for i in 0..inner.k {
        let angle = inner.aa + i as f32 * inner.an;

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
        current_length *= inner.ra;
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

pub fn ui_elements(inner: &mut ParamsInner, ui: &mut Ui) -> bool {
    add_number_slider(ui, "k", &mut inner.k, 1..=2500)
        || add_float_slider_pi(ui, "an", &mut inner.an)
        || add_float_slider(ui, "ra", &mut inner.ra, 0.0..=1.0)
        || add_float_slider_pi(ui, "aa", &mut inner.aa)
        || add_float_slider_np_length(ui, "rr", &mut inner.rr)
}
