use crate::{DesignParams, Model, Segment, Shape, Shapes};
use nannou::prelude::*;
use nannou_egui::egui::Ui;
use std::f32::consts::PI;
use ui_controlled_params::UiControlledParams;

#[derive(UiControlledParams)]
pub struct ParamsInner {
    #[param(range(2..=14))]
    pub n: u32, // depth of recursion
    pub l0_fn: Box<dyn Fn(u32) -> f32>, // base length
    #[param(pi)]
    pub a0_factor: f32,
    pub a0_fn: Box<dyn Fn(u32, f32) -> f32>, // initial angle
    pub p0_fn: Box<dyn Fn() -> Point2>,      // initial position
    pub rules_fn: Box<dyn Fn(u32) -> Vec<i32>>, // turning rules
}

pub struct Params {
    pub inner: ParamsInner,
    pub calculate_shapes: Box<dyn Fn(&ParamsInner) -> Shapes>,
    pub ui_elements: UiElements,
}

pub fn model(app: &App, inner: ParamsInner) -> Model {
    let params = DesignParams::Dragon(Params {
        inner,
        calculate_shapes: Box::new(calculate_shapes),
        ui_elements: Box::new(ParamsInner::ui_elements),
    });

    crate::model(params, app)
}

pub fn calculate_shapes(inner: &ParamsInner) -> Shapes {
    let mut shapes = Shapes::new();
    let mut shape = Shape::new();
    let mut segment = Segment::new();

    let rules = (inner.rules_fn)(inner.n);
    let p0 = (inner.p0_fn)();
    let l0 = (inner.l0_fn)(inner.n);

    segment.push(p0);

    let mut p0 = p0;
    let mut p1 = p0;
    let mut p2 = p0;

    let mut current_angle = (inner.a0_fn)(inner.n, inner.a0_factor);

    let nn = 2_i32.pow(inner.n) - 1;

    fn step_segment(p0: &mut Point2, p1: &mut Point2, p2: &mut Point2, step: Point2) {
        *p0 = *p1;
        *p1 = *p2;
        *p2 += step;
    }

    for i in 0..=nn {
        if i == 0 {
            step_segment(
                &mut p0,
                &mut p1,
                &mut p2,
                pt2(l0 * current_angle.cos(), l0 * current_angle.sin()),
            );
        } else {
            let mut ii = i;
            let mut j = 0;

            while ii % 2 == 0 {
                ii /= 2;
                j += 1;
            }

            let aa = (rules[inner.n as usize - j] * 2 - 1) as f32
                * ((((ii - 1) / 2) % 2) * 2 - 1) as f32
                * PI
                / 2.0;
            current_angle += aa;

            step_segment(
                &mut p0,
                &mut p1,
                &mut p2,
                pt2(l0 * current_angle.cos(), l0 * current_angle.sin()),
            );
        }

        segment.push((p0 + pt2(3.0, 3.0) * p1) / pt2(4.0, 4.0));
        segment.push((p2 + pt2(3.0, 3.0) * p1) / pt2(4.0, 4.0));
    }

    shape.push(segment);
    shapes.push(shape);

    shapes
}
