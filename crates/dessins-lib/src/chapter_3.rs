use crate::{Segment, Shape, Shapes, NP};
use nannou::prelude::*;
use nannou_egui::egui::Ui;
use std::f32::consts::PI;
use ui_controlled_params::UiControlledParams;

#[derive(UiControlledParams)]
#[params(Dragon)]
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

impl ParamsInner {
    pub fn calculate_shapes(&self) -> Shapes {
        let mut shapes = Shapes::new();
        let mut shape = Shape::new();
        let mut segment = Segment::new();

        let rules = (self.rules_fn)(self.n);
        let p0 = (self.p0_fn)();
        let l0 = (self.l0_fn)(self.n);

        segment.push(p0);

        let mut p0 = p0;
        let mut p1 = p0;
        let mut p2 = p0;

        let mut current_angle = (self.a0_fn)(self.n, self.a0_factor);

        let nn = 2_i32.pow(self.n) - 1;

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

                let aa = (rules[self.n as usize - j] * 2 - 1) as f32
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
}

impl Default for Params {
    fn default() -> Self {
        Self {
            inner: ParamsInner {
                n: 6,
                a0_fn: Box::new(|n: u32, factor: f32| (n - 2) as f32 * factor),
                a0_factor: -PI / 4.0,
                l0_fn: Box::new(|n: u32| NP as f32 / (2.0_f32.sqrt().powf(n as f32))),
                p0_fn: Box::new(|| pt2(-(NP as f32) / 6.0, -(NP as f32) / 2.5)),
                rules_fn: Box::new(|n: u32| vec![0; n as usize + 1]),
            },
            calculate_shapes: Box::new(ParamsInner::calculate_shapes),
            ui_elements: Box::new(ParamsInner::ui_elements),
        }
    }
}
