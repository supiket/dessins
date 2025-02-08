use crate::{Segment, Shape, Shapes};
use nannou::prelude::*;
use nannou_egui::egui::Ui;
use std::f32::consts::PI;
use ui_controlled_params::UiControlledParams;

#[derive(UiControlledParams)]
#[params(Orbital)]
pub struct ParamsInner {
    #[param(range(1000..=6000))]
    pub n: u32, // # segments
    #[param(range(1..=600))]
    pub t1: u32, // # times the planet turns around the sun
    #[param(range(1..=600))]
    pub t2: u32, // # times the satellite turns around the planet
    #[param(np, length)]
    pub r1: f32, // radius of the planet's curve
    #[param(range(1..=4))]
    pub k1: u32, // elliptic parameter of the planet's curve
    #[param(range(1..=4))]
    pub k2: u32, // elliptic parameter of the planet's curve
    pub r2_eq: Box<dyn Fn(R2Params) -> f32>,
}

pub struct R2Params {
    pub i: f32,
    pub n: f32,
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
