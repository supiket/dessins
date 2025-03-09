use crate::{
    adjustable_dessin::AdjustableDessin,
    adjustable_variable::types::{
        f32::{F32Variant, F32},
        pt2::Pt2,
        u32::U32,
    },
    shapes::{Segment, Shape, Shapes, NP},
};
use adjustable_dessin_derive::DefaultAdjustableDessin;
use nannou::prelude::*;

#[derive(Clone, Debug, PartialEq, Reflect, DefaultAdjustableDessin)]
#[reflect(Default)]
pub struct FractalStar {
    pub n: U32,
    pub k: U32,
    pub ra: F32,
    pub ll: F32,
    pub aa: F32,
    pub p0: Pt2,
    pub a0: F32,
}

impl FractalStar {
    pub fn calculate_shapes(&mut self) -> Shapes {
        let mut shapes = Shapes::new();
        let mut shape = Shape::new();
        let mut segment = Segment::new();

        let mut p0 = self.p0.get_value();
        let mut a0 = self.a0.get_value();

        let n = self.n.get_value();
        let k = self.k.get_value();

        let nn = n * (n - 1).pow(k - 1) - 1;

        for i in 0..=nn {
            let mut i1 = i;
            let mut h = 0;

            while i1 % (n - 1) == 0 && h < (k - 1) {
                i1 /= n - 1;
                h += 1;
            }

            let l0 = self.ll.get_value() * self.ra.get_value().powf((k - 1 - h) as f32);
            a0 += self.aa.get_value();

            let point = p0 + pt2(l0 * a0.cos(), l0 * a0.sin());

            segment.push(point);
            p0 = point;
        }

        shape.push(segment);
        shapes.push(shape);

        shapes
    }
}

impl Default for FractalStar {
    fn default() -> Self {
        let aa = F32::new(4.0 / 5.0, F32Variant::Angle);
        let ll = F32::new(1.0, F32Variant::Length);

        let mut a0 = aa.clone();
        a0.set_value(-a0.get_value());
        let p0 = Pt2::new(pt2((-ll.get_value()) / 2.0, (NP as f32) * (0.5)));

        Self {
            n: U32::new(5, 3..=20, 1),
            k: U32::new(5, 2..=12, 1),
            ra: F32::new_from_range(0.35, 0.2..=1.8),
            ll,
            aa,
            a0,
            p0,
        }
    }
}
