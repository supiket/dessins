use crate::{
    adjustable_dessin::AdjustableDessin,
    adjustable_variable::types::{F32Variant, F32, U32},
    shapes::{Segment, Shape, Shapes},
};
use adjustable_dessin_derive::DefaultAdjustableDessin;
use nannou::prelude::*;

#[derive(Clone, Debug, PartialEq, Reflect, DefaultAdjustableDessin)]
#[reflect(Default)]
pub struct Star {
    pub k: U32,  // # vertices
    pub h: U32,  // # vertices to skip (clockwise) before connecting two dots
    pub r: F32,  // radius of the circle C on which the vertices are
    pub ad: F32, // angle (in radians) of the vector CS with horizontal, where S is the first vertex
}

impl Star {
    pub fn calculate_shapes(&mut self) -> Shapes {
        let mut shapes = Shapes::new();
        let mut shape = Shape::new();
        let mut segment = Segment::new();

        for i in 0..self.k.get_value() {
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
        let h = self.h.get_value() as f32;
        let k = self.k.get_value() as f32;
        let ad = self.ad.get_value();

        let angle = (2.0 * i * h * PI) / k + ad;
        let x = self.r.get_value() * angle.cos();
        let y = self.r.get_value() * angle.sin();
        pt2(x, y)
    }
}

impl Default for Star {
    fn default() -> Self {
        Self {
            k: U32::new(5, 5..=100),
            h: U32::new(3, 3..=5),
            r: F32::new(0.45, F32Variant::Length),
            ad: F32::new(0.5, F32Variant::Angle),
        }
    }
}
