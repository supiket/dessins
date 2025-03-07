use crate::{
    meta::f32::F32,
    reflect::ControllableParams,
    shapes::{Segment, Shape, Shapes, NP},
};
use nannou::prelude::*;

#[derive(Clone, Debug, PartialEq, Reflect)]
#[reflect(Default)]
pub struct Spiral {
    pub n: F32, // # segments
    pub t: F32, // # times the planet turns around the sun
    pub r: F32, // flattening parameter of the ellipse
    pub l: F32, // decrease factor beween the first ellipse traveled and the last
    pub an_factor: F32,
}

impl Spiral {
    pub fn calculate_shapes(&mut self) -> Shapes {
        let mut shapes = Shapes::new();
        let mut shape = Shape::new();
        let mut segment = Segment::new();

        let np = NP as f32;
        let n = self.n.value;
        let t = self.t.value;
        let r = self.r.value;
        let l = self.l.value;
        let an_factor = self.an_factor.value;

        for i in 0..=self.n.value as usize {
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

impl ControllableParams for Spiral {}

impl Default for Spiral {
    fn default() -> Self {
        Self {
            n: F32::new_from_range(2000.0, 1000.0..=9000.0),
            t: F32::new_from_range(40.0, 40.0..=60.0),
            r: F32::new_from_range(0.8, 0.1..=2.0),
            l: F32::new_from_range(0.1, 0.1..=2.0),
            an_factor: F32::new_from_range(1.0, 1.0..=4.0),
        }
    }
}
