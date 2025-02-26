use crate::{Segment, Shape, Shapes, NP};
use nannou::prelude::*;
use ui_controlled_params::UiControlledParams;

#[derive(UiControlledParams)]
#[params(Star)]
pub struct ParamsInner {
    #[param(label = "star k", range(5..=100))]
    pub k: u32, // # vertices
    #[param(label = "star h", range(3..=50))]
    pub h: u32, // # vertices to skip (clockwise) before connecting two dots
    #[param(label = "star r", length)]
    pub r: f32, // radius of the circle C on which the vertices are
    #[param(label = "star ad", pi)]
    pub ad: f32, // angle (in radians) of the vector CS with horizontal, where S is the first vertex
}

impl ParamsInner {
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
            inner: ParamsInner {
                k: 5,
                h: 3,
                r: NP as f32 * 0.45,
                ad: PI / 2.0,
            },
            calculate_shapes: Box::new(ParamsInner::calculate_shapes),
            ui_elements: Box::new(ParamsInner::ui_elements),
        }
    }
}
