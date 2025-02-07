use crate::{
    add_float_slider_np_length, add_number_slider, DesignParams, Model, Segment, Shape, Shapes,
};
use nannou::prelude::*;
use nannou_egui::egui::Ui;
use std::f32::consts::PI;

pub struct ParamsInner {
    pub n: u32,  // # segments
    pub t1: u32, // # times the planet turns around the sun
    pub t2: u32, // # times the satellite turns around the planet
    pub r1: f32, // radius of the planet's curve
    pub k1: u32, // elliptic parameter of the planet's curve
    pub k2: u32, // elliptic parameter of the planet's curve
    pub r2_eq: Box<dyn Fn(R2Params) -> f32>,
}

pub struct R2Params {
    pub i: f32,
    pub n: f32,
}

pub struct Params {
    pub inner: ParamsInner,
    pub calculate_shapes: Box<dyn Fn(&ParamsInner) -> Shapes>,
    pub ui_elements: Box<dyn Fn(&mut ParamsInner, &mut Ui) -> bool>,
}

pub fn model(app: &App, inner: ParamsInner) -> Model {
    let params = DesignParams::Orbital(Params {
        inner,
        calculate_shapes: Box::new(calculate_shapes),
        ui_elements: Box::new(ui_elements),
    });

    crate::model(params, app)
}

pub fn ui_elements(inner: &mut ParamsInner, ui: &mut Ui) -> bool {
    add_number_slider(ui, "curve n", &mut inner.n, 1000..=6000)
        || add_number_slider(ui, "curve t1", &mut inner.t1, 1..=600)
        || add_number_slider(ui, "curve t2", &mut inner.t2, 1..=600)
        || add_float_slider_np_length(ui, "curve r1", &mut inner.r1)
        || add_number_slider(ui, "curve k1", &mut inner.k1, 1..=4)
        || add_number_slider(ui, "curve k2", &mut inner.k2, 1..=4)
}

pub fn calculate_shapes(inner: &ParamsInner) -> Shapes {
    let mut shapes = Shapes::new();
    let mut shape = Shape::new();
    let mut segment = Segment::new();

    let n = inner.n as f32;
    let t1 = inner.t1 as f32;
    let t2 = inner.t2 as f32;
    let r1 = inner.r1;
    let k1 = inner.k1 as f32;
    let k2 = inner.k2 as f32;

    for i in 0..=inner.n {
        let i = i as f32;
        let r2 = (inner.r2_eq)(R2Params { i, n });
        let a1 = 2.0 * PI * i / n * t1;
        let a2 = 2.0 * PI * i / n * t2;

        let x = r1 * (k1 * a1).cos() + r2 * a2.cos();
        let y = r1 * (k2 * a1).sin() + r2 * a2.sin();

        segment.push(pt2(x, y));
    }

    shape.push(segment);
    shapes.push(shape);

    shapes
}
