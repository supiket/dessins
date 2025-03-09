use crate::{
    adjustable_dessin::AdjustableDessin,
    adjustable_variable::types::{expression_f32::ExpressionF32, u32::U32},
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
    #[reflect(ignore)]
    pub r1: ExpressionF32,
    #[reflect(ignore)]
    pub r2: ExpressionF32,
}

impl Stick {
    pub fn calculate_shapes(&mut self) -> Shapes {
        let mut shapes = Shapes::new();
        let mut shape = Shape::new();

        let n = self.n.value;
        let k = self.k.value;

        for i in 0..=self.m.value {
            self.r1.ctx.insert("i".to_string(), i as f32);
            self.r1.ctx_ext.remove("i");
            self.r1.val = evalexpr::eval_number_with_context(
                &self.r1.expr,
                &ExpressionF32::evaluatable_ctx(&self.r1.ctx),
            )
            .unwrap_or_else(|_| {
                self.r1.expr = Self::default_r1_expr();
                evalexpr::eval_number_with_context(
                    &self.r1.expr,
                    &ExpressionF32::evaluatable_ctx(&self.r1.ctx),
                )
                .expect("default expression has to evaluate")
            }) as f32;
            let r1 = self.r1.val;

            self.r2.ctx.insert("i".to_string(), i as f32);
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

        let ctx = HashMap::from([
            ("n".to_string(), n as f32),
            ("k".to_string(), k as f32),
            ("pi".to_string(), PI),
        ]);

        let r1 = ExpressionF32 {
            expr: Stick::default_r1_expr(),
            ctx: ctx.clone(),
            ctx_ext: HashMap::from([("i".to_string(), ())]),
            val: 120.0,
        };
        let r2 = ExpressionF32 {
            expr: Stick::default_r2_expr(),
            ctx,
            ctx_ext: HashMap::from([("i".to_string(), ())]),
            val: 100.0,
        };

        Self {
            n: U32::new(n, 10..=600, 1),
            m: U32::new(1, 1..=6, 1),
            k: U32::new(k, 1..=7, 1),
            r1,
            r2,
        }
    }
}
