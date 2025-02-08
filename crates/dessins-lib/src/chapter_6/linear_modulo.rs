use crate::{Segment, Shape, Shapes, NP};
use nannou::prelude::*;
use nannou_egui::egui::Ui;
use std::f32::consts::PI;
use ui_controlled_params::UiControlledParams;

#[derive(UiControlledParams)]
#[params(LinearModulo)]
pub struct ParamsInner {
    #[param(range(10..=400))]
    pub n: u32,
    #[param(range(10..=400))]
    pub m: u32,
    #[param(range(1.0..=5.0))]
    pub k1: f32,
    #[param(range(1.0..=5.0))]
    pub k2: f32,
    #[param(range(1..=10))]
    pub h: u32,
    #[param(range(1..=8))]
    pub i1_factor: u32,
    pub y_eq: Box<dyn Fn(&YParams) -> f32>,
}

pub struct YParams {
    pub i: f32,
    pub n: f32,
    pub k2: f32,
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
