use crate::{ui::ExpressionF32, Shape, Shapes};
use evalexpr::{ContextWithMutableVariables, HashMapContext};
use nannou::prelude::*;
use std::{collections::HashMap, f32::consts::PI};
use ui_controlled_params::UiControlledParams;

#[derive(UiControlledParams)]
#[params(LinearSticks)]
pub struct ParamsInner {
    #[param(range(10..=600))]
    pub n: u32,
    #[param(range(1..=6))]
    pub m: u32,
    #[param(range(1..=7))]
    pub k: u32,
    #[param(range(-1000.0..=1000.0), expression(default="120", context(n, k, ext(i, pi))))]
    pub r1: ExpressionF32,
    #[param(range(-1000.0..=1000.0), expression(default="100", context(n, k, ext(i, pi))))]
    pub r2: ExpressionF32,
}

pub struct RParams {
    pub i: f32,
}

impl ParamsInner {
    pub fn calculate_shapes(&mut self) -> Shapes {
        let mut shapes = Shapes::new();
        let mut shape = Shape::new();

        let n = self.n as f32;
        let k = self.k as f32;

        for i in 0..=self.m {
            self.r1
                .ctx
                .set_value("i".to_string(), evalexpr::Value::Float(i as f64))
                .unwrap();
            self.r1.ctx_ext.remove("i");
            self.r1.val =
                evalexpr::eval_number_with_context(&self.r1.expr, &self.r1.ctx).unwrap() as f32;
            let r1 = self.r1.val;

            self.r2
                .ctx
                .set_value("i".to_string(), evalexpr::Value::Float(i as f64))
                .unwrap();
            self.r2.ctx_ext.remove("i");
            self.r2.val =
                evalexpr::eval_number_with_context(&self.r2.expr, &self.r2.ctx).unwrap() as f32;
            let r2 = self.r2.val;

            for j in 0..self.n {
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
}

impl Default for Params {
    fn default() -> Self {
        let n = 100;
        let k = 5;

        let mut ctx = HashMapContext::new();
        ctx.set_value("n".to_string(), evalexpr::Value::Float(n as f64))
            .unwrap();
        ctx.set_value("k".to_string(), evalexpr::Value::Float(k as f64))
            .unwrap();
        ctx.set_value("pi".to_string(), evalexpr::Value::Float(f64::PI()))
            .unwrap();

        let r1 = ExpressionF32 {
            expr: "120".to_string(),
            ctx: ctx.clone(),
            ctx_ext: HashMap::from([("i".to_string(), ())]),
            val: 120.0,
        };
        let r2 = ExpressionF32 {
            expr: "100".to_string(),
            ctx,
            ctx_ext: HashMap::from([("i".to_string(), ())]),
            val: 100.0,
        };

        Self {
            inner: ParamsInner { n, m: 1, k, r1, r2 },
            calculate_shapes: Box::new(ParamsInner::calculate_shapes),
            ui_elements: Box::new(ParamsInner::ui_elements),
        }
    }
}
