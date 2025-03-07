use crate::{
    meta::{
        expression_f32::ExpressionF32,
        f32::{F32Variant, F32},
    },
    reflect::ControllableParams,
    shapes::{Segment, Shape, Shapes},
};
use nannou::prelude::*;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Reflect)]
#[reflect(Default)]
pub struct Orbital {
    pub n: F32,  // # segments
    pub t1: F32, // # times the planet turns around the sun
    pub t2: F32, // # times the satellite turns around the planet
    pub r1: F32, // radius of the planet's curve
    pub k1: F32, // elliptic parameter of the planet's curve
    pub k2: F32, // elliptic parameter of the planet's curve
    #[reflect(ignore)]
    pub r2: ExpressionF32,
}

pub struct R2Orbital {
    pub i: f32,
    pub n: f32,
}

impl Orbital {
    pub fn calculate_shapes(&mut self) -> Shapes {
        let mut shapes = Shapes::new();
        let mut shape = Shape::new();
        let mut segment = Segment::new();

        let n = self.n.value;
        let t1 = self.t1.value;
        let t2 = self.t2.value;
        let r1 = self.r1.value;
        let k1 = self.k1.value;
        let k2 = self.k2.value;

        for i in 0..=self.n.value as usize {
            let i = i as f32;

            self.r2.ctx.insert("i".to_string(), i);
            self.r2.ctx_ext.remove("i");
            self.r2.val = evalexpr::eval_number_with_context(
                &self.r2.expr,
                &ExpressionF32::evaluatable_ctx(&self.r2.ctx),
            )
            .unwrap_or_else(|_| {
                self.r2.expr = Self::default_r2_expr();
                evalexpr::eval_number_with_context(
                    &self.r2.expr,
                    &ExpressionF32::evaluatable_ctx(&self.r2.ctx),
                )
                .expect("default expression has to evaluate")
            }) as f32;

            let r2 = self.r2.val;
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

    fn default_r2_expr() -> String {
        "96.0 * (1 - i / n)".to_string()
    }
}

impl ControllableParams for Orbital {}

impl Default for Orbital {
    fn default() -> Self {
        let n = 2000;
        let ctx = HashMap::from([("n".to_string(), n as f32)]);
        let r2 = ExpressionF32 {
            expr: Orbital::default_r2_expr(),
            ctx,
            ctx_ext: HashMap::from([("i".to_string(), ())]),
            val: 0.0,
        };
        Self {
            n: F32::new_from_range(n as f32, 1000.0..=6000.0),
            t1: F32::new_from_range(2.0, 1.0..=600.0),
            t2: F32::new_from_range(100.0, 1.0..=600.0),
            r1: F32::new(0.25, F32Variant::Length),
            k1: F32::new_from_range(1.0, 1.0..=4.0),
            k2: F32::new_from_range(1.0, 1.0..=4.0),
            r2,
        }
    }
}
