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
pub struct Rotating {
    pub n: F32,  // # segments
    pub t1: F32, // # times the planet turns around the sun
    pub t2: F32, // # times the satellite turns around the planet
    pub r1: F32, // radius of the planet's curve
    pub k1: F32, // elliptic parameter of the planet's curve
    pub k2: F32, // elliptic parameter of the planet's curve
    pub r2: F32, // radius of the satellite's curve
    pub h1: F32, // elliptic parameter of the satellite's curve
    pub h2: F32, // elliptic parameter of the satellite's curve
    #[reflect(ignore)]
    pub s: ExpressionF32,
}

impl Rotating {
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
        let r2 = self.r2.value;
        let h1 = self.h1.value;
        let h2 = self.h2.value;

        for i in 0..=self.n.value as usize {
            let i = i as f32;

            self.s.ctx.insert("i".to_string(), i);
            self.s.ctx_ext.remove("i");
            self.s.val = evalexpr::eval_number_with_context(
                &self.s.expr,
                &ExpressionF32::evaluatable_ctx(&self.s.ctx),
            )
            .unwrap_or_else(|_| {
                self.s.expr = Self::default_s_expr();
                evalexpr::eval_number_with_context(
                    &self.s.expr,
                    &ExpressionF32::evaluatable_ctx(&self.s.ctx),
                )
                .expect("default expression has to evaluate")
            }) as f32;
            let s = self.s.val;
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

    fn default_s_expr() -> String {
        "math::cos(4.0 * pi * i / n) * 0.4 + 0.6".to_string()
    }
}

impl ControllableParams for Rotating {}

impl Default for Rotating {
    fn default() -> Self {
        let n = 2000;
        let ctx = HashMap::from([("n".to_string(), n as f32), ("pi".to_string(), PI)]);
        let s = ExpressionF32 {
            expr: Rotating::default_s_expr(),
            ctx,
            ctx_ext: HashMap::from([("i".to_string(), ())]),
            val: 1.0,
        };
        Self {
            n: F32::new_from_range(n as f32, 1000.0..=6000.0),
            t1: F32::new_from_range(1.0, 0.5..=600.0),
            t2: F32::new_from_range(100.0, 0.5..=600.0),
            r1: F32::new(1.0 / 6.0, F32Variant::Length),
            k1: F32::new_from_range(1.0, 1.0..=4.0),
            k2: F32::new_from_range(1.0, 1.0..=4.0),
            r2: F32::new(1.0 / 4.0, F32Variant::Length),
            h1: F32::new_from_range(1.0, 1.0..=4.0),
            h2: F32::new_from_range(1.0, 1.0..=4.0),
            s,
        }
    }
}
