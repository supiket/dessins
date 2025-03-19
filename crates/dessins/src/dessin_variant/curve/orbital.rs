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
pub struct Orbital {
    pub n: U32,  // # segments
    pub t1: U32, // # times the planet turns around the sun
    pub t2: U32, // # times the satellite turns around the planet
    pub r1: F32, // radius of the planet's curve
    pub k1: U32, // elliptic parameter of the planet's curve
    pub k2: U32, // elliptic parameter of the planet's curve
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

        let n = self.n.get_value() as f32;
        let t1 = self.t1.get_value() as f32;
        let t2 = self.t2.get_value() as f32;
        let r1 = self.r1.get_value();
        let k1 = self.k1.get_value() as f32;
        let k2 = self.k2.get_value() as f32;

        for i in 0..=n as usize {
            let i = i as f32;

            self.r2.insert_ctx_entry("n", self.n.get_value() as f32);
            self.r2.insert_ctx_entry("i", i);
            let r2 = self.r2.eval_expr();

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

impl Default for Orbital {
    fn default() -> Self {
        let n = 2000;
        let ctx = Context::new(&[("n".to_string(), n as f32)]);
        let r2 = ExpressionF32::new(
            Orbital::default_r2_expr(),
            Orbital::default_r2_expr(),
            ctx,
            HashMap::from([("i".to_string(), ())]),
            0.0,
            0.0..=480.0,
            0.1,
        );
        Self {
            n: U32::new(n, 1000..=6000),
            t1: U32::new(2, 1..=600),
            t2: U32::new(100, 1..=600),
            r1: F32::new(0.25, F32Variant::Length),
            k1: U32::new(1, 1..=4),
            k2: U32::new(1, 1..=4),
            r2,
        }
    }
}
