use crate::{
    add_float_slider_np_length, add_float_slider_pi, add_number_slider, DesignParams, Model,
    Segment, Shape, Shapes,
};
use nannou::prelude::*;
use nannou_egui::egui::Ui;

#[derive(Clone)]
pub struct ParamsInner {
    pub k: u32,  // # vertices
    pub h: u32,  // # vertices to skip (clockwise) before connecting two dots
    pub r: f32,  // radius of the circle C on which the vertices are
    pub ad: f32, // angle (in radians) of the vector CS with horizontal, where S is the first vertex
}

pub struct Params {
    pub inner: ParamsInner,
    pub calculate_shapes: Box<dyn Fn(&ParamsInner) -> Shapes>,
    pub ui_elements: Box<dyn Fn(&mut ParamsInner, &mut Ui) -> bool>,
}

pub fn model(app: &App, inner: ParamsInner) -> Model {
    let params = DesignParams::Star(Params {
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

    for i in 0..inner.k {
        let point = calculate_point(inner, i);
        segment.push(point);
    }

    segment.push(segment[0]);

    shape.push(segment);
    shapes.push(shape);
    shapes
}

pub fn calculate_point(inner: &ParamsInner, i: u32) -> Point2 {
    let angle = (2.0 * i as f32 * inner.h as f32 * PI) / inner.k as f32 + inner.ad;
    let x = inner.r * angle.cos();
    let y = inner.r * angle.sin();
    pt2(x, y)
}

pub fn ui_elements(inner: &mut ParamsInner, ui: &mut Ui) -> bool {
    add_number_slider(ui, "star k", &mut inner.k, 5..=100)
        || add_number_slider(ui, "star h", &mut inner.h, 3..=50)
        || add_float_slider_np_length(ui, "star r", &mut inner.r)
        || add_float_slider_pi(ui, "star ad", &mut inner.ad)
}
