use crate::{
    adjustable_dessin::AdjustableDessin,
    adjustable_variable::types::{expression_f32::ExpressionF32, f32::F32},
    shapes::{Shape, Shapes},
};
use adjustable_dessin_derive::DefaultAdjustableDessin;
use nannou::prelude::*;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Reflect, DefaultAdjustableDessin)]
#[reflect(Default)]
pub struct Stick {
    pub n: F32,
    pub m: F32,
    pub k: F32,
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

        for i in 0..=self.m.value as usize {
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

            for j in 0..self.n.value as usize {
                let j = j as f32;

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
        let n = 100.0;
        let k = 5.0;

        let ctx = HashMap::from([
            ("n".to_string(), n),
            ("k".to_string(), k),
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
            n: F32::new_from_range(n, 10.0..=600.0),
            m: F32::new_from_range(1.0, 1.0..=6.0),
            k: F32::new_from_range(k, 1.0..=7.0),
            r1,
            r2,
        }
    }
}
