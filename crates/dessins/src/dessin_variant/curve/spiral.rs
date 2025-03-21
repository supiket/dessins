use crate::{
    adjustable_dessin::AdjustableDessin,
    adjustable_variable::types::{F32Variant, F32, U32},
    shapes::{Segment, Shape, Shapes, NP},
};
use adjustable_dessin_derive::DefaultAdjustableDessin;
use nannou::prelude::*;

#[derive(Clone, Debug, PartialEq, Reflect, DefaultAdjustableDessin)]
#[reflect(Default)]
pub struct Spiral {
    pub n: U32, // # segments
    pub t: U32, // # times the planet turns around the sun
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
        let n = self.n.get_value() as f32;
        let t = self.t.get_value() as f32;
        let r = self.r.get_value();
        let l = self.l.get_value();
        let an_factor = self.an_factor.get_value();

        for i in 0..=n as usize {
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

impl Default for Spiral {
    fn default() -> Self {
        Self {
            n: U32::new(2000, 1000..=9000),
            t: U32::new(40, 40..=60),
            r: F32::new(0.8, F32Variant::None(0.1..=2.0)),
            l: F32::new(0.1, F32Variant::None(0.1..=2.0)),
            an_factor: F32::new(1.0, F32Variant::None(1.0..=4.0)),
        }
    }
}
