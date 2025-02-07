use crate::{add_float_slider, add_number_slider, DesignParams, Model, Segment, Shape, Shapes, NP};
use nannou::prelude::*;
use nannou_egui::egui::Ui;
use std::f32::consts::PI;

pub struct ParamsInner {
    pub n: u32,
    pub m: u32,
    pub k1: f32,
    pub k2: f32,
    pub h: u32,
    pub i1_factor: u32,
    pub y_eq: Box<dyn Fn(&YParams) -> f32>,
}

pub struct YParams {
    pub i: f32,
    pub n: f32,
    pub k2: f32,
}

pub struct Params {
    pub inner: ParamsInner,
    pub calculate_shapes: Box<dyn Fn(&ParamsInner) -> Shapes>,
    pub ui_elements: Box<dyn Fn(&mut ParamsInner, &mut Ui) -> bool>,
}

pub fn model(app: &App, inner: ParamsInner) -> Model {
    let params = DesignParams::LinearModulo(Params {
        inner,
        calculate_shapes: Box::new(calculate_shapes),
        ui_elements: Box::new(ui_elements),
    });

    crate::model(params, app)
}

pub fn ui_elements(inner: &mut ParamsInner, ui: &mut Ui) -> bool {
    add_number_slider(ui, "linear modulo n", &mut inner.n, 10..=400)
        || add_number_slider(ui, "linear modulo m", &mut inner.m, 10..=400)
        || add_float_slider(ui, "linear modulo k1", &mut inner.k1, 1.0..=5.0)
        || add_float_slider(ui, "linear modulo k2", &mut inner.k2, 1.0..=5.0)
        || add_number_slider(ui, "linear modulo h", &mut inner.h, 1..=10)
        || add_number_slider(ui, "linear modulo i1 factor", &mut inner.i1_factor, 1..=8)
}

pub fn calculate_shapes(inner: &ParamsInner) -> Shapes {
    let mut shapes = Shapes::new();
    let mut shape = Shape::new();

    let points = calculate_points(inner);

    for i in 0..=inner.m {
        let start_index = ((inner.i1_factor * i) % inner.n) as usize;
        let end_index = ((inner.h * i) % inner.n) as usize;

        let segment = vec![points[start_index], points[end_index]];
        shape.push(segment);
    }

    shapes.push(shape);

    shapes
}

fn calculate_points(inner: &ParamsInner) -> Segment {
    let mut points = vec![];

    let n = inner.n as f32;
    let k1 = inner.k1;
    let k2 = inner.k2;

    for i in 0..=inner.n {
        let i = i as f32;

        let x = NP as f32 * 0.5 * (k1 * i * PI / n).sin();
        let y = (inner.y_eq)(&YParams { i, n, k2 });
        points.push(pt2(x, y));
    }

    points
}
