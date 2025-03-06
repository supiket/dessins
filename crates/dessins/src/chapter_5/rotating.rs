use crate::{ui::ExpressionF32, Segment, Shape, Shapes, NP};
use evalexpr::{ContextWithMutableVariables, HashMapContext};
use nannou::prelude::*;
use std::{collections::HashMap, f32::consts::PI};
use ui_controlled_params::UiControlledParams;

#[derive(UiControlledParams)]
#[params(Rotating)]
pub struct ParamsInner {
    #[param(range(1000..=6000))]
    pub n: u32, // # segments
    #[param(range(0.5..=600.0))]
    pub t1: f32, // # times the planet turns around the sun
    #[param(range(0.5..=600.0))]
    pub t2: f32, // # times the satellite turns around the planet
    #[param(length)]
    pub r1: f32, // radius of the planet's curve
    #[param(range(1..=4))]
    pub k1: u32, // elliptic parameter of the planet's curve
    #[param(range(1..=4))]
    pub k2: u32, // elliptic parameter of the planet's curve
    #[param(length)]
    pub r2: f32, // radius of the satellite's curve
    #[param(range(1..=4))]
    pub h1: u32, // elliptic parameter of the satellite's curve
    #[param(range(1..=4))]
    pub h2: u32, // elliptic parameter of the satellite's curve
    #[param(range(0.6..=1.0), expression(default="math::cos(4.0 * pi * i / n) * 0.4 + 0.6", context(n, ext(i, pi))))]
    pub s: ExpressionF32,
}

impl ParamsInner {
    pub fn calculate_shapes(&mut self) -> Shapes {
        let mut shapes = Shapes::new();
        let mut shape = Shape::new();
        let mut segment = Segment::new();

        let n = self.n as f32;
        let t1 = self.t1;
        let t2 = self.t2;
        let r1 = self.r1;
        let k1 = self.k1 as f32;
        let k2 = self.k2 as f32;
        let r2 = self.r2;
        let h1 = self.h1 as f32;
        let h2 = self.h2 as f32;

        for i in 0..=self.n {
            let i = i as f32;

            self.s
                .ctx
                .set_value("i".to_string(), evalexpr::Value::Float(i as f64))
                .expect("setting to value of same type each time");
            self.s.ctx_ext.remove("i");
            self.s.val = evalexpr::eval_number_with_context(&self.s.expr, &self.s.ctx)
                .unwrap_or_else(|_| {
                    self.s.expr = Self::default_s_expr();
                    evalexpr::eval_number_with_context(&self.s.expr, &self.s.ctx)
                        .expect("default expression has to evaluate")
                }) as f32;
            let s = self.s.val;
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

impl Default for Params {
    fn default() -> Self {
        let n = 2000;
        let mut ctx = HashMapContext::new();
        ctx.set_value("n".to_string(), evalexpr::Value::Float(n as f64))
            .expect("setting to value of same type each time");
        ctx.set_value("pi".to_string(), evalexpr::Value::Float(f64::PI()))
            .expect("setting to value of same type each time");
        let s = ExpressionF32 {
            expr: ParamsInner::default_s_expr(),
            ctx,
            ctx_ext: HashMap::from([("i".to_string(), ())]),
            val: 1.0,
        };
        Self {
            inner: ParamsInner {
                n,
                t1: 1.0,
                t2: 100.0,
                r1: NP as f32 / 6.0,
                k1: 1,
                k2: 1,
                r2: NP as f32 / 4.0,
                h1: 1,
                h2: 1,
                s,
            },
            calculate_shapes: Box::new(ParamsInner::calculate_shapes),
            ui_elements: Box::new(ParamsInner::ui_elements),
        }
    }
}
