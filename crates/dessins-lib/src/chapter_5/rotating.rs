use crate::{Segment, Shape, Shapes};
use nannou::prelude::*;
use nannou_egui::egui::Ui;
use std::f32::consts::PI;
use ui_controlled_params::UiControlledParams;

#[derive(UiControlledParams)]
#[params(Rotating)]
pub struct ParamsInner {
    #[param(range(1000..=6000))]
    pub n: u32, // # segments
    #[param(range(0.5..=600.0))]
    pub t1: f32, // # times the planet turns around the sun
    #[param(range(0.5..=600.0))]
    pub t2: f32, // # times the satellite turns around the planet
    #[param(np, length)]
    pub r1: f32, // radius of the planet's curve
    #[param(range(1..=4))]
    pub k1: u32, // elliptic parameter of the planet's curve
    #[param(range(1..=4))]
    pub k2: u32, // elliptic parameter of the planet's curve
    #[param(np, length)]
    pub r2: f32, // radius of the satellite's curve
    #[param(range(1..=4))]
    pub h1: u32, // elliptic parameter of the satellite's curve
    #[param(range(1..=4))]
    pub h2: u32, // elliptic parameter of the satellite's curve
    pub s_eq: Box<dyn Fn(SParams) -> f32>,
}

pub struct SParams {
    pub i: f32,
    pub n: f32,
}

pub fn calculate_shapes(inner: &ParamsInner) -> Shapes {
    let mut shapes = Shapes::new();
    let mut shape = Shape::new();
    let mut segment = Segment::new();

    let n = inner.n as f32;
    let t1 = inner.t1;
    let t2 = inner.t2;
    let r1 = inner.r1;
    let k1 = inner.k1 as f32;
    let k2 = inner.k2 as f32;
    let r2 = inner.r2;
    let h1 = inner.h1 as f32;
    let h2 = inner.h2 as f32;

    for i in 0..=inner.n {
        let i = i as f32;

        let s = (inner.s_eq)(SParams { i, n });
        let an = 2.0 * PI * i / n;
        let c1 = (h1 * an * t1).cos();
        let s1 = (h2 * an * t1).sin();
        let c2 = s * (k1 * an * t2).cos();
        let s2 = s * (k2 * an * t2).sin();

        let x = r1 * c1 + r2 * (c1 * c2 - s1 * s2);
        let y = r1 * s1 + r2 * (s1 * c2 + c1 * s2);

        segment.push(pt2(x, y));
    }

    shape.push(segment);
    shapes.push(shape);

    shapes
}
