use crate::{AnimationState, Segment, Shape, Shapes, NP};
use nannou::prelude::*;

#[derive(Clone, Reflect)]
pub struct Params {
    pub k: u32,  // # vertices
    pub r: f32,  // radius of the circle on which the vertices are
    pub ad: f32, // angle (in radians) of the vector CS with horizontal, where S is the first vertex
    #[reflect(ignore)]
    pub k_animation: Option<AnimationState>,
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
        let angle = (2.0 * i as f32 * PI) / self.k as f32 + self.ad;
        let x = self.r * angle.cos();
        let y = self.r * angle.sin();
        pt2(x, y)
    }
}

impl Default for Params {
    fn default() -> Self {
        Self {
            k: 3,
            r: NP as f32 * 0.45,
            ad: 0_f32,
            k_animation: None,
        }
    }
}
