use crate::{
    adjustable_dessin::AdjustableDessin,
    adjustable_variable::types::f32::{F32Variant, F32},
    shapes::{Segment, Shape, Shapes, NP},
};
use nannou::prelude::*;

#[derive(Clone, Debug, PartialEq, Reflect)]
#[reflect(Default)]
pub struct Fractal {
    pub n: F32,
    pub k: F32,
    pub ra: F32,
    pub ll: F32,
    pub aa: F32,
    #[reflect(ignore)]
    pub p0: Point2,
    #[reflect(ignore)]
    pub a0: F32,
}

impl Fractal {
    pub fn calculate_shapes(&mut self) -> Shapes {
        let mut shapes = Shapes::new();
        let mut shape = Shape::new();
        let mut segment = Segment::new();

        let mut p0 = self.p0;
        let mut a0 = self.a0.value;

        let nn = self.n.value as u32 * (self.n.value as u32 - 1).pow(self.k.value as u32 - 1) - 1;

        for i in 0..=nn {
            let mut i1 = i;
            let mut h = 0;

            while i1 % (self.n.value as u32 - 1) == 0 && h < (self.k.value as u32 - 1) {
                i1 /= self.n.value as u32 - 1;
                h += 1;
            }

            let l0 = self.ll.value * self.ra.value.powf((self.k.value as u32 - 1 - h) as f32);
            a0 += self.aa.value;

            let point = p0 + pt2(l0 * a0.cos(), l0 * a0.sin());

            segment.push(point);
            p0 = point;
        }

        shape.push(segment);
        shapes.push(shape);

        shapes
    }
}

impl AdjustableDessin for Fractal {}

impl Default for Fractal {
    fn default() -> Self {
        let aa = F32::new(4.0 / 5.0, F32Variant::Angle);
        let ll = F32::new(1.0, F32Variant::Length);

        let mut a0 = aa.clone();
        a0.value = -a0.value;
        let p0 = pt2((-ll.value) / 2.0, (NP as f32) * (0.5));

        Self {
            n: F32::new_from_range(5.0, 3.0..=20.0),
            k: F32::new_from_range(5.0, 2.0..=12.0),
            ra: F32::new_from_range(0.35, 0.2..=1.8),
            ll,
            aa,
            a0,
            p0,
        }
    }
}
