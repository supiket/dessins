use crate::{
    adjustable_dessin::AdjustableDessin,
    adjustable_variable::types::{expression_f32::ExpressionF32, f32::F32, u32::U32},
    shapes::{Segment, Shape, Shapes, NP},
};
use adjustable_dessin_derive::DefaultAdjustableDessin;
use nannou::prelude::*;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Reflect, DefaultAdjustableDessin)]
#[reflect(Default)]
pub struct Modulo {
    pub n: U32,
    pub m: U32,
    pub k1: F32,
    pub k2: F32,
    pub h: U32,
    pub i1_factor: U32,
    #[reflect(ignore)]
    pub y: ExpressionF32,
}

impl Modulo {
    pub fn calculate_shapes(&mut self) -> Shapes {
        let mut shapes = Shapes::new();
        let mut shape = Shape::new();

        let points = self.calculate_points();

        for i in 0..=self.m.value {
            let start_index = ((self.i1_factor.value * i) % self.n.value) as usize;
            let end_index = ((self.h.value * i) % self.n.value) as usize;

            let segment = vec![points[start_index], points[end_index]];
            shape.push(segment);
        }

        shapes.push(shape);

        shapes
    }

    fn calculate_points(&mut self) -> Segment {
        let mut points = vec![];

        let n = self.n.value as f32;
        let k1 = self.k1.value;

        for i in 0..=self.n.value {
            let i = i as f32;

            let x = NP as f32 * 0.5 * (k1 * i * PI / n).sin();
            self.y.ctx.insert("i".to_string(), i);
            self.y.ctx_ext.remove("i");
            self.y.val = evalexpr::eval_number_with_context(
                &self.y.expr,
                &ExpressionF32::evaluatable_ctx(&self.y.ctx),
            )
            .unwrap_or_else(|_| {
                self.y.expr = Self::default_y_expr();
                evalexpr::eval_number_with_context(
                    &self.y.expr,
                    &ExpressionF32::evaluatable_ctx(&self.y.ctx),
                )
                .expect("default expression has to evaluate")
            }) as f32;
            let y = self.y.val;

            points.push(pt2(x, y));
        }

        points
    }

    fn default_y_expr() -> String {
        "360 * math::cos(k2 * i * pi / n)".to_string()
    }
}

impl Default for Modulo {
    fn default() -> Self {
        let n = 400;
        let k2 = 5.0;

        let ctx = HashMap::from([
            ("n".to_string(), n as f32),
            ("k2".to_string(), k2),
            ("pi".to_string(), PI),
        ]);

        let y = ExpressionF32 {
            expr: Modulo::default_y_expr(),
            ctx,
            ctx_ext: HashMap::from([("i".to_string(), ())]),
            val: 360.0,
        };

        Self {
            n: U32::new(n, 10..=400, 1),
            m: U32::new(400, 10..=400, 1),
            k1: F32::new_from_range(4.0, 1.0..=5.0),
            k2: F32::new_from_range(5.0, 1.0..=5.0),
            h: U32::new(2, 1..=10, 1),
            i1_factor: U32::new(1, 1..=8, 1),
            y,
        }
    }
}
