use crate::{
    adjustable_dessin::AdjustableDessin,
    adjustable_variable::types::{Context, ExpressionF32, F32Variant, F32, U32},
    shapes::{Segment, Shape, Shapes},
};
use adjustable_dessin_derive::DefaultAdjustableDessin;
use nannou::prelude::*;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Reflect, DefaultAdjustableDessin)]
#[reflect(Default)]
pub struct Rotating {
    pub n: U32,  // # segments
    pub t1: F32, // # times the planet turns around the sun
    pub t2: F32, // # times the satellite turns around the planet
    pub r1: F32, // radius of the planet's curve
    pub k1: U32, // elliptic parameter of the planet's curve
    pub k2: U32, // elliptic parameter of the planet's curve
    pub r2: F32, // radius of the satellite's curve
    pub h1: U32, // elliptic parameter of the satellite's curve
    pub h2: U32, // elliptic parameter of the satellite's curve
    pub s: ExpressionF32,
}

impl Rotating {
    pub fn calculate_shapes(&mut self) -> Shapes {
        let mut shapes = Shapes::new();
        let mut shape = Shape::new();
        let mut segment = Segment::new();

        let n = self.n.get_value() as f32;
        let t1 = self.t1.get_value();
        let t2 = self.t2.get_value();
        let r1 = self.r1.get_value();
        let k1 = self.k1.get_value() as f32;
        let k2 = self.k2.get_value() as f32;
        let r2 = self.r2.get_value();
        let h1 = self.h1.get_value() as f32;
        let h2 = self.h2.get_value() as f32;

        for i in 0..=n as usize {
            let i = i as f32;

            self.s.insert_ctx_entry("n", self.n.get_value() as f32);
            self.s.insert_ctx_entry("i", i);
            let s = self.s.eval_expr();

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

impl Default for Rotating {
    fn default() -> Self {
        let n = 2000;
        let ctx = Context::new(&[("n".to_string(), n as f32), ("pi".to_string(), PI)]);
        let s = ExpressionF32::new(
            Rotating::default_s_expr(),
            Rotating::default_s_expr(),
            ctx,
            HashMap::from([("i".to_string(), ())]),
            1.0,
            0.6..=1.0,
            0.1,
        );
        Self {
            n: U32::new(n, 1000..=6000),
            t1: F32::new(1.0, F32Variant::None(0.5..=600.0)),
            t2: F32::new(100.0, F32Variant::None(0.5..=600.0)),
            r1: F32::new(1.0 / 6.0, F32Variant::Length),
            k1: U32::new(1, 1..=4),
            k2: U32::new(1, 1..=4),
            r2: F32::new(1.0 / 4.0, F32Variant::Length),
            h1: U32::new(1, 1..=4),
            h2: U32::new(1, 1..=4),
            s,
        }
    }
}
