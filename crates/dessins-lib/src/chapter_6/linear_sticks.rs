use crate::{Shape, Shapes};
use nannou::prelude::*;
use nannou_egui::egui::Ui;
use std::f32::consts::PI;
use ui_controlled_params::UiControlledParams;

#[derive(UiControlledParams)]
#[params(LinearSticks)]
pub struct ParamsInner {
    #[param(range(10..=600))]
    pub n: u32,
    #[param(range(1..=6))]
    pub m: u32,
    #[param(range(1..=7))]
    pub k: u32,
    pub r1_eq: Box<dyn Fn(&RParams) -> f32>,
    pub r2_eq: Box<dyn Fn(&RParams) -> f32>,
}

pub struct RParams {
    pub i: f32,
}

pub fn calculate_shapes(inner: &ParamsInner) -> Shapes {
    let mut shapes = Shapes::new();
    let mut shape = Shape::new();

    let n = inner.n as f32;
    let k = inner.k as f32;

    for i in 0..=inner.m {
        let r_params = RParams { i: i as f32 };
        let r1 = (inner.r1_eq)(&r_params);
        let r2 = (inner.r2_eq)(&r_params);

        for j in 0..inner.n {
            let j = j as f32;

            let an = 2.0 * j * PI / n;

            let x = r1 * an.cos() + r2 * (k * an).cos();
            let y = r1 * an.sin() + r2 * (k * an).sin();
            let d = pt2(x, y);

            let x = r1 * an.cos() + r2 * (k * an + PI).cos();
            let y = r1 * an.sin() + r2 * (k * an + PI).sin();
            let a = pt2(x, y);

            let segment = vec![d, a];
            shape.push(segment);
        }
    }

    shapes.push(shape);

    shapes
}
