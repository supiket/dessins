use crate::{Segment, Shape, Shapes, NP};
use nannou::prelude::*;

#[derive(Clone)]
pub struct Params {
    pub k: u32,  // # vertices
    pub h: u32,  // # vertices to skip (clockwise) before connecting two dots
    pub r: f32,  // radius of the circle C on which the vertices are
    pub ad: f32, // angle (in radians) of the vector CS with horizontal, where S is the first vertex
}

impl Params {
    pub fn calculate_shapes(&mut self) -> Shapes {
        let mut shapes = Shapes::default();
        let mut shape = Shape::new();
        let mut segment = Segment::new();

        for i in 0..self.k {
            let point = self.calculate_point(i);
            segment.push(point);
        }

        segment.push(segment[0]);

        shape.push(segment);
        shapes.push(shape);
        shapes
    }

    pub fn calculate_point(&self, i: u32) -> Point2 {
        let angle = (2.0 * i as f32 * self.h as f32 * PI) / self.k as f32 + self.ad;
        let x = self.r * angle.cos();
        let y = self.r * angle.sin();
        pt2(x, y)
    }
}

impl Default for Params {
    fn default() -> Self {
        Self {
            k: 5,
            h: 3,
            r: NP as f32 * 0.45,
            ad: PI / 2.0,
        }
    }
}
