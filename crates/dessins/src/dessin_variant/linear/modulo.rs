use crate::{
    adjustable_dessin::AdjustableDessin,
    adjustable_variable::types::{
        expression_f32::{Context, ExpressionF32},
        f32::{F32Variant, F32},
        u32::U32,
    },
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
    pub y: ExpressionF32,
}

impl Modulo {
    pub fn calculate_shapes(&mut self) -> Shapes {
        let mut shapes = Shapes::new();
        let mut shape = Shape::new();

        let points = self.calculate_points();

        for i in 0..=self.m.get_value() {
            let start_index = ((self.i1_factor.get_value() * i) % self.n.get_value()) as usize;
            let end_index = ((self.h.get_value() * i) % self.n.get_value()) as usize;

            let segment = vec![points[start_index], points[end_index]];
            shape.push(segment);
        }

        shapes.push(shape);

        shapes
    }

    fn calculate_points(&mut self) -> Segment {
        let mut points = vec![];

        let n = self.n.get_value() as f32;
        let k1 = self.k1.get_value();

        for i in 0..=n as usize {
            let i = i as f32;

            let x = NP as f32 * 0.5 * (k1 * i * PI / n).sin();

            self.y.insert_ctx_entry("n", self.n.get_value() as f32);
            self.y.insert_ctx_entry("k2", self.k2.get_value());
            self.y.set_ext_ctx("i", i);
            let y = self.y.eval_expr();

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

        let ctx = Context::new(&[
            ("n".to_string(), n as f32),
            ("k2".to_string(), k2),
            ("pi".to_string(), PI),
        ]);

        let y = ExpressionF32::new(
            Modulo::default_y_expr(),
            Modulo::default_y_expr(),
            ctx,
            HashMap::from([("i".to_string(), ())]),
            360.0,
            -360.0..=360.0,
            1.0,
        );

        Self {
            n: U32::new(n, 10..=400),
            m: U32::new(400, 10..=400),
            k1: F32::new(4.0, F32Variant::None(1.0..=5.0)),
            k2: F32::new(5.0, F32Variant::None(1.0..=5.0)),
            h: U32::new(2, 1..=10),
            i1_factor: U32::new(1, 1..=8),
            y,
        }
    }
}
