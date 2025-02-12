use crate::{ui::ExpressionF32, Segment, Shape, Shapes, NP};
use evalexpr::{ContextWithMutableVariables, HashMapContext};
use nannou::prelude::*;
use std::{collections::HashMap, f32::consts::PI};
use ui_controlled_params::UiControlledParams;

#[derive(UiControlledParams)]
#[params(LinearModulo)]
pub struct ParamsInner {
    #[param(range(10..=400))]
    pub n: u32,
    #[param(range(10..=400))]
    pub m: u32,
    #[param(range(1.0..=5.0))]
    pub k1: f32,
    #[param(range(1.0..=5.0))]
    pub k2: f32,
    #[param(range(1..=10))]
    pub h: u32,
    #[param(range(1..=8))]
    pub i1_factor: u32,
    #[param(range(-360.0..=360.0), expression(default="360 * math::cos(k2 * i * pi / n)", context(n, k2, ext(i, pi))))]
    pub y: ExpressionF32,
}

impl ParamsInner {
    pub fn calculate_shapes(&mut self) -> Shapes {
        let mut shapes = Shapes::new();
        let mut shape = Shape::new();

        let points = self.calculate_points();

        for i in 0..=self.m {
            let start_index = ((self.i1_factor * i) % self.n) as usize;
            let end_index = ((self.h * i) % self.n) as usize;

            let segment = vec![points[start_index], points[end_index]];
            shape.push(segment);
        }

        shapes.push(shape);

        shapes
    }

    fn calculate_points(&mut self) -> Segment {
        let mut points = vec![];

        let n = self.n as f32;
        let k1 = self.k1;

        for i in 0..=self.n {
            let i = i as f32;

            let x = NP as f32 * 0.5 * (k1 * i * PI / n).sin();
            self.y
                .ctx
                .set_value("i".to_string(), evalexpr::Value::Float(i as f64))
                .unwrap();
            self.y.ctx_ext.remove("i");
            self.y.val =
                evalexpr::eval_number_with_context(&self.y.expr, &self.y.ctx).unwrap() as f32;
            let y = self.y.val;

            points.push(pt2(x, y));
        }

        points
    }
}

impl Default for Params {
    fn default() -> Self {
        let n = 400;
        let k2 = 5.0;

        let mut ctx = HashMapContext::new();
        ctx.set_value("n".to_string(), evalexpr::Value::Float(n as f64))
            .unwrap();
        ctx.set_value("k2".to_string(), evalexpr::Value::Float(k2 as f64))
            .unwrap();
        ctx.set_value("pi".to_string(), evalexpr::Value::Float(f64::PI()))
            .unwrap();

        let y = ExpressionF32 {
            expr: "360 * math::cos(k2 * i * pi / n)".to_string(),
            ctx,
            ctx_ext: HashMap::from([("i".to_string(), ())]),
            val: 360.0,
        };

        Self {
            inner: ParamsInner {
                n,
                m: 400,
                k1: 4.0,
                k2,
                h: 2,
                i1_factor: 1,
                y,
            },
            calculate_shapes: Box::new(ParamsInner::calculate_shapes),
            ui_elements: Box::new(ParamsInner::ui_elements),
        }
    }
}
