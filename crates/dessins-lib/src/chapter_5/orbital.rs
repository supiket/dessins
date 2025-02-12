use crate::{ui::ExpressionF32, Segment, Shape, Shapes, NP};
use evalexpr::{ContextWithMutableVariables, HashMapContext};
use nannou::prelude::*;
use std::{collections::HashMap, f32::consts::PI};
use ui_controlled_params::UiControlledParams;

#[derive(UiControlledParams)]
#[params(Orbital)]
pub struct ParamsInner {
    #[param(range(1000..=6000))]
    pub n: u32, // # segments
    #[param(range(1..=600))]
    pub t1: u32, // # times the planet turns around the sun
    #[param(range(1..=600))]
    pub t2: u32, // # times the satellite turns around the planet
    #[param(length)]
    pub r1: f32, // radius of the planet's curve
    #[param(range(1..=4))]
    pub k1: u32, // elliptic parameter of the planet's curve
    #[param(range(1..=4))]
    pub k2: u32, // elliptic parameter of the planet's curve
    #[param(range(0.0..=480.0), expression(default="96.0 * (1 - i / n)", context(n, ext(i))))]
    pub r2: ExpressionF32,
}

pub struct R2Params {
    pub i: f32,
    pub n: f32,
}

impl ParamsInner {
    pub fn calculate_shapes(&mut self) -> Shapes {
        let mut shapes = Shapes::new();
        let mut shape = Shape::new();
        let mut segment = Segment::new();

        let n = self.n as f32;
        let t1 = self.t1 as f32;
        let t2 = self.t2 as f32;
        let r1 = self.r1;
        let k1 = self.k1 as f32;
        let k2 = self.k2 as f32;

        for i in 0..=self.n {
            let i = i as f32;

            self.r2
                .ctx
                .set_value("i".to_string(), evalexpr::Value::Float(i as f64))
                .unwrap();
            self.r2.ctx_ext.remove("i");
            self.r2.val =
                evalexpr::eval_number_with_context(&self.r2.expr, &self.r2.ctx).unwrap() as f32;

            let r2 = self.r2.val;
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
}

impl Default for Params {
    fn default() -> Self {
        let n = 2000;
        let mut ctx = HashMapContext::new();
        ctx.set_value("n".to_string(), evalexpr::Value::Float(n as f64))
            .unwrap();
        let r2 = ExpressionF32 {
            expr: "96.0 * (1 - i / n)".to_string(),
            ctx,
            ctx_ext: HashMap::from([("i".to_string(), ())]),
            val: 0.0,
        };
        Self {
            inner: ParamsInner {
                n,
                t1: 2,
                t2: 100,
                r1: NP as f32 * 0.25,
                k1: 1,
                k2: 1,
                r2,
            },
            calculate_shapes: Box::new(ParamsInner::calculate_shapes),
            ui_elements: Box::new(ParamsInner::ui_elements),
        }
    }
}
