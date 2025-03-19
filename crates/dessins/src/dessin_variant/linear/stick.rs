use crate::{
    adjustable_dessin::AdjustableDessin,
    adjustable_variable::types::{Context, ExpressionF32, U32},
    shapes::{Shape, Shapes},
};
use adjustable_dessin_derive::DefaultAdjustableDessin;
use nannou::prelude::*;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Reflect, DefaultAdjustableDessin)]
#[reflect(Default)]
pub struct Stick {
    pub n: U32,
    pub m: U32,
    pub k: U32,
    pub r1: ExpressionF32,
    pub r2: ExpressionF32,
}

impl Stick {
    pub fn calculate_shapes(&mut self) -> Shapes {
        let mut shapes = Shapes::new();
        let mut shape = Shape::new();

        let n = self.n.get_value();
        let k = self.k.get_value();

        for i in 0..=self.m.get_value() {
            self.r1.insert_ctx_entry("n", self.n.get_value() as f32);
            self.r1.insert_ctx_entry("k", self.k.get_value() as f32);
            self.r2.insert_ctx_entry("n", self.n.get_value() as f32);
            self.r2.insert_ctx_entry("k", self.k.get_value() as f32);

            self.r1.insert_ctx_entry("i", i as f32);
            let r1 = self.r1.eval_expr();

            self.r2.insert_ctx_entry("i", i as f32);
            let r2 = self.r2.eval_expr();

            for j in 0..n {
                let j = j as f32;
                let n = n as f32;
                let k = k as f32;

                let an = 2.0 * j * PI / n;

                let x = r1 * an.cos() + r2 * (k * an).cos();
                let y = r1 * an.sin() + r2 * (k * an).sin();
                let d = pt2(x, y);

                let x = r1 * an.cos() + r2 * (k * an + PI).cos();
                let y = r1 * an.sin() + r2 * (k * an + PI).sin();
                let a = pt2(x, y);

                let segment = vec![d, a];
                shape.push(segment);
            }
        }

        shapes.push(shape);

        shapes
    }

    fn default_r1_expr() -> String {
        "120.0".to_string()
    }

    fn default_r2_expr() -> String {
        "100.0".to_string()
    }
}

impl Default for Stick {
    fn default() -> Self {
        let n = 100;
        let k = 5;

        let ctx = Context::new(&[
            ("n".to_string(), n as f32),
            ("k".to_string(), k as f32),
            ("pi".to_string(), PI),
        ]);

        let r1 = ExpressionF32::new(
            Stick::default_r1_expr(),
            Stick::default_r1_expr(),
            ctx.clone(),
            HashMap::from([("i".to_string(), ())]),
            120.0,
            -1000.0..=1000.0,
            10.0,
        );
        let r2 = ExpressionF32::new(
            Stick::default_r2_expr(),
            Stick::default_r2_expr(),
            ctx.clone(),
            HashMap::from([("i".to_string(), ())]),
            100.0,
            -1000.0..=1000.0,
            10.0,
        );

        Self {
            n: U32::new(n, 10..=600),
            m: U32::new(1, 1..=6),
            k: U32::new(k, 1..=7),
            r1,
            r2,
        }
    }
}
