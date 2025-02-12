use crate::{ui::ExpressionF32, Segment, Shape, Shapes, NP};
use evalexpr::{ContextWithMutableVariables, HashMapContext};
use nannou::prelude::*;
use std::f32::consts::PI;
use ui_controlled_params::UiControlledParams;

#[derive(UiControlledParams)]
#[params(Dragon)]
pub struct ParamsInner {
    #[param(range(2..=14))]
    pub n: u32, // depth of recursion
    #[param(range(0.0..= 300.0), expression(default="480.0 / (math::sqrt(2.0) ^ n", context(n)))]
    pub l0: ExpressionF32, // initial length
    #[param(range(-180.0..=180.0), expression(default="-pi / 4 * (n-2)", context(n, ext(pi))))]
    pub a0: ExpressionF32, // initial length
    #[param]
    pub p0: Point2, // initial position
    #[param(range(0..=1))]
    pub rules: Vec<i32>, // turning rules
}

impl ParamsInner {
    pub fn calculate_shapes(&mut self) -> Shapes {
        let mut shapes = Shapes::new();
        let mut shape = Shape::new();
        let mut segment = Segment::new();

        if self.n as usize != self.rules.len() + 1 {
            self.rules = vec![0; self.n as usize + 1];
            self.l0.val =
                evalexpr::eval_number_with_context(&self.l0.expr, &self.l0.ctx).unwrap() as f32;
            self.a0.val =
                evalexpr::eval_number_with_context(&self.a0.expr, &self.a0.ctx).unwrap() as f32;
        }

        let p0 = self.p0;
        let l0 = self.l0.val;
        let a0 = self.a0.val;

        segment.push(p0);

        let mut p0 = p0;
        let mut p1 = p0;
        let mut p2 = p0;

        let mut current_angle = a0;

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

                let aa = (self.rules[self.n as usize - j] * 2 - 1) as f32
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
        let n = 6;
        let rules = vec![0; n as usize + 1];
        let l0_fn = |n: u32| NP as f32 / (2.0_f32.sqrt().powf(n as f32));
        let a0_fn = |n: u32| (n - 2) as f32 * -PI / 4.0;
        let p0_fn = || pt2(-(NP as f32) / 6.0, -(NP as f32) / 2.5);

        let mut ctx = HashMapContext::new();
        ctx.set_value("n".to_string(), evalexpr::Value::Float(n as f64))
            .unwrap();
        let l0 = ExpressionF32 {
            expr: "480 / (math::sqrt(2.0) ^ n)".to_string(),
            ctx: ctx.clone(),
            ctx_ext: Default::default(),
            val: l0_fn(n),
        };
        let mut ctx = ctx.clone();
        ctx.set_value("pi".to_string(), evalexpr::Value::Float(f64::PI()))
            .unwrap();
        let a0 = ExpressionF32 {
            expr: "-pi / 4 * (n - 2)".to_string(),
            ctx,
            ctx_ext: Default::default(),
            val: a0_fn(n),
        };
        Self {
            inner: ParamsInner {
                n,
                l0,
                a0,
                p0: p0_fn(),
                rules,
            },
            calculate_shapes: Box::new(ParamsInner::calculate_shapes),
            ui_elements: Box::new(ParamsInner::ui_elements),
        }
    }
}
