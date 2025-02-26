use crate::{Segment, Shape, Shapes, NP};
use nannou::prelude::*;
use std::f32::consts::PI;
use ui_controlled_params::UiControlledParams;

#[derive(UiControlledParams)]
#[params(Spiral)]
pub struct ParamsInner {
    #[param(range(1000..=9000))]
    pub n: u32, // # segments
    #[param(range(40..=60))]
    pub t: u32, // # times the planet turns around the sun
    #[param(length)]
    pub r: f32, // flattening parameter of the ellipse
    #[param(length)]
    pub l: f32, // decrease factor beween the first ellipse traveled and the last
    #[param(range(1.0..=4.0))]
    pub an_factor: f32,
}

impl ParamsInner {
    pub fn calculate_shapes(&mut self) -> Shapes {
        let mut shapes = Shapes::default();
        let mut shape = Shape::new();
        let mut segment = Segment::new();

        let np = NP as f32;
        let n = self.n as f32;
        let t = self.t as f32;
        let r = self.r;
        let l = self.l;
        let an_factor = self.an_factor;

        for i in 0..=self.n {
            let i = i as f32;

            let rr = l.powf(i / n);
            let an = 2.0 * PI * i / n * an_factor;

            let x = rr * (t * an).cos();
            let y = rr * r * (t * an).sin();

            let co = an.cos();
            let si = an.sin();

            let xx = x * co - y * si;
            let yy = x * si + y * co;

            let x = xx * np / 2.0;
            let y = yy * np / 2.0;

            segment.push(pt2(x, y));
        }

        shape.push(segment);
        shapes.push(shape);

        shapes
    }
}

impl Default for Params {
    fn default() -> Self {
        Self {
            inner: ParamsInner {
                n: 2000,
                t: 40,
                r: 0.8,
                l: 0.1,
                an_factor: 1.0,
            },
            calculate_shapes: Box::new(ParamsInner::calculate_shapes),
            ui_elements: Box::new(ParamsInner::ui_elements),
        }
    }
}
