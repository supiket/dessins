use crate::{
    meta::{expression_f32::ExpressionF32, f32::F32},
    reflect::ControllableParams,
    shapes::{Segment, Shape, Shapes, NP},
};
use nannou::prelude::*;
use std::collections::HashMap;

#[derive(Clone, Debug, Reflect)]
#[reflect(Default)]
pub struct Dragon {
    pub n: F32, // depth of recursion
    #[reflect(ignore)]
    pub l0: ExpressionF32, // initial length
    #[reflect(ignore)]
    pub a0: ExpressionF32, // initial length
    #[reflect(ignore)]
    pub p0: Point2, // initial position
    #[reflect(ignore)]
    pub rules: Vec<i32>, // turning rules TODO: #[param(range(0..=1))]
}

impl Dragon {
    pub fn calculate_shapes(&mut self) -> Shapes {
        let mut shapes = Shapes::new();
        let mut shape = Shape::new();
        let mut segment = Segment::new();

        if self.n.value as usize != self.rules.len() + 1 {
            self.rules = vec![0; self.n.value as usize + 1];
            self.l0.val = evalexpr::eval_number_with_context(
                &self.l0.expr,
                &ExpressionF32::evaluatable_ctx(&self.l0.ctx),
            )
            .unwrap_or_else(|_| {
                self.l0.expr = Self::default_l0_expr();
                evalexpr::eval_number_with_context(
                    &self.l0.expr,
                    &ExpressionF32::evaluatable_ctx(&self.l0.ctx),
                )
                .expect("default expression has to evaluate")
            }) as f32;
            self.a0.val = evalexpr::eval_number_with_context(
                &self.a0.expr,
                &ExpressionF32::evaluatable_ctx(&self.a0.ctx),
            )
            .unwrap_or_else(|_| {
                self.a0.expr = Self::default_a0_expr();
                evalexpr::eval_number_with_context(
                    &self.a0.expr,
                    &ExpressionF32::evaluatable_ctx(&self.a0.ctx),
                )
                .expect("default expression has to evaluate")
            }) as f32;
        }

        let p0 = self.p0;
        let l0 = self.l0.val;
        let a0 = self.a0.val;

        segment.push(p0);

        let mut p0 = p0;
        let mut p1 = p0;
        let mut p2 = p0;

        let mut current_angle = a0;

        let nn = 2_i32.pow(self.n.value as u32) - 1;
        self.rules.resize(self.n.value as usize + 1, 0);

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

                let aa = (self.rules[self.n.value as usize - j] * 2 - 1) as f32
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

    fn default_l0_expr() -> String {
        "480 / (math::sqrt(2.0) ^ n)".to_string()
    }

    fn default_a0_expr() -> String {
        "-pi / 4 * (n - 2)".to_string()
    }
}

impl ControllableParams for Dragon {}

impl Default for Dragon {
    fn default() -> Self {
        let n = 6;
        let rules = vec![0; n as usize + 1];
        let l0_fn = |n: u32| NP as f32 / (2.0_f32.sqrt().powf(n as f32));
        let a0_fn = |n: u32| (n - 2) as f32 * -PI / 4.0;
        let p0_fn = || pt2(-(NP as f32) / 6.0, -(NP as f32) / 2.5);

        let ctx = HashMap::from([("n".to_string(), n as f32)]);
        let l0 = ExpressionF32::new(Self::default_l0_expr(), ctx, Default::default(), l0_fn(n));

        let ctx = HashMap::from([("n".to_string(), n as f32), ("pi".to_string(), PI)]);
        let a0 = ExpressionF32::new(Self::default_a0_expr(), ctx, Default::default(), a0_fn(n));

        Self {
            n: F32::new_from_range(n as f32, 2.0..=14.0),
            l0,
            a0,
            p0: p0_fn(),
            rules,
        }
    }
}
