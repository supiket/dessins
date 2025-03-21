use crate::{
    adjustable_dessin::AdjustableDessin,
    adjustable_variable::types::{Context, ExpressionF32, U32},
    shapes::{Segment, Shape, Shapes, NP},
};
use adjustable_dessin_derive::DefaultAdjustableDessin;
use nannou::prelude::*;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Reflect, DefaultAdjustableDessin)]
#[reflect(Default)]
pub struct PolarCurve {
    pub n: U32, // # segments
    pub a: ExpressionF32,
    pub r: ExpressionF32,
}

impl PolarCurve {
    pub fn calculate_shapes(&mut self) -> Shapes {
        let mut shapes = Shapes::new();
        let mut shape = Shape::new();
        let mut segment = Segment::new();

        let n = self.n.get_value() as f32;

        for i in 0..=n as usize {
            let i = i as f32;

            self.a.insert_ctx_entry("n", self.n.get_value() as f32);
            self.a.insert_ctx_entry("i", i);
            let a = self.a.eval_expr();

            self.r.insert_ctx_entry("n", self.n.get_value() as f32);
            self.r.insert_ctx_entry("i", i);
            let r = self.r.eval_expr();

            let x = r * a.cos();
            let y = r * a.sin();

            let x = x * NP as f32 / 2.0;
            let y = y * NP as f32 / 2.0;

            segment.push(pt2(x, y));
        }

        shape.push(segment);
        shapes.push(shape);

        shapes
    }

    fn default_a_expr() -> String {
        "3 * pi / 4 * math::sin(30*pi*i/n) * math::sin(pi*i/n)".to_string()
    }

    fn default_r_expr() -> String {
        "i/n".to_string()
    }
}

impl Default for PolarCurve {
    fn default() -> Self {
        let n = 2000;
        let ctx = Context::new(&[("n".to_string(), n as f32), ("pi".to_string(), PI)]);
        let a = ExpressionF32::new(
            PolarCurve::default_a_expr(),
            PolarCurve::default_a_expr(),
            ctx.clone(),
            HashMap::from([("i".to_string(), ())]),
            0.0,
            0.0..=480.0,
            0.1,
        );
        let r = ExpressionF32::new(
            PolarCurve::default_r_expr(),
            PolarCurve::default_r_expr(),
            ctx,
            HashMap::from([("i".to_string(), ())]),
            0.0,
            0.0..=480.0,
            0.1,
        );
        Self {
            n: U32::new(n, 400..=6000),
            a,
            r,
        }
    }
}
