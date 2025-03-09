use crate::{
    adjustable_dessin::AdjustableDessin,
    adjustable_variable::types::{
        f32::{F32Variant, F32},
        u32::U32,
    },
    shapes::{Segment, Shape, Shapes},
};
use adjustable_dessin_derive::DefaultAdjustableDessin;
use nannou::prelude::*;

#[derive(Clone, Debug, PartialEq, Reflect, DefaultAdjustableDessin)]
#[reflect(Default)]
pub struct Polygon {
    pub k: U32,  // # vertices TODO: reconsider f32, at least discretize behaviour
    pub r: F32,  // radius of the circle on which the vertices are
    pub ad: F32, // angle (in radians) of the vector CS with horizontal, where S is the first vertex,
}

impl Polygon {
    pub fn calculate_shapes(&mut self) -> Shapes {
        let mut shapes = Shapes::new();
        let mut shape = Shape::new();
        let mut segment = Segment::new();

        for i in 0..self.k.value {
            let point = self.calculate_point(i);
            segment.push(point);
        }

        segment.push(segment[0]);

        shape.push(segment);
        shapes.push(shape);
        shapes
    }

    pub fn calculate_point(&self, i: u32) -> Point2 {
        let i = i as f32;
        let k = self.k.value as f32;
        let ad = self.ad.value;

        let angle = (2.0 * i * PI) / k + ad;
        let x = self.r.value * angle.cos();
        let y = self.r.value * angle.sin();
        pt2(x, y)
    }
}

impl Default for Polygon {
    fn default() -> Self {
        Self {
            k: U32::new(3, 3..=20, 1),
            r: F32::new(0.45, F32Variant::Length),
            ad: F32::new(0.0, F32Variant::Angle),
        }
    }
}
