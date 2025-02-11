use crate::{Segment, Shape, Shapes, NP};
use nannou::prelude::*;
use std::f32::consts::PI;
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
    // TODO: contains i
    pub y_eq: Box<dyn Fn(&YParams) -> f32 + Send + Sync>,
}

pub struct YParams {
    pub i: f32,
    pub n: f32,
    pub k2: f32,
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

    fn calculate_points(&self) -> Segment {
        let mut points = vec![];

        let n = self.n as f32;
        let k1 = self.k1;
        let k2 = self.k2;

        for i in 0..=self.n {
            let i = i as f32;

            let x = NP as f32 * 0.5 * (k1 * i * PI / n).sin();
            let y = (self.y_eq)(&YParams { i, n, k2 });
            points.push(pt2(x, y));
        }

        points
    }
}

impl Default for Params {
    fn default() -> Self {
        Self {
            inner: ParamsInner {
                n: 400,
                m: 400,
                k1: 4.0,
                k2: 5.0,
                h: 2,
                i1_factor: 1,
                y_eq: Box::new(|params: &YParams| {
                    NP as f32 * 0.75 * (params.k2 * params.i * PI / params.n).cos()
                }),
            },
            calculate_shapes: Box::new(ParamsInner::calculate_shapes),
            ui_elements: Box::new(ParamsInner::ui_elements),
        }
    }
}
