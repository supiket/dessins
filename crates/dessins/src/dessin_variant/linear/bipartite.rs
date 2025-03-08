use crate::{
    adjustable_dessin::AdjustableDessin,
    adjustable_variable::types::f32::F32,
    shapes::{Segment, Shape, Shapes, NP},
};
use nannou::prelude::*;

pub type OuterSegment = Segment;
pub type InnerSegment = Segment;
use adjustable_dessin_derive::DefaultAdjustableDessin;

#[derive(Clone, Debug, PartialEq, Reflect, DefaultAdjustableDessin)]
#[reflect(Default)]
pub struct Bipartite {
    pub n: F32,
    #[reflect(ignore)]
    pub a: Point2,
    #[reflect(ignore)]
    pub b: Point2,
    #[reflect(ignore)]
    pub c: Point2,
    #[reflect(ignore)]
    pub d: Point2,
}

pub struct YBipartite {
    pub i: f32,
    pub n: f32,
    pub k2: f32,
}

impl Bipartite {
    pub fn calculate_shapes(&mut self) -> Shapes {
        let mut shapes = Shapes::new();
        let mut shape = Shape::new();

        let (outer_points, inner_points) = self.calculate_points();

        for outer in &outer_points {
            for inner in &inner_points {
                let segment = vec![*outer, *inner];
                shape.push(segment);
            }
        }

        shapes.push(shape);

        shapes
    }

    pub fn calculate_points(&self) -> (OuterSegment, InnerSegment) {
        let mut outer_segment = vec![];
        let mut inner_segment = vec![];

        let n = self.n.value;

        for i in 0..=self.n.value as usize {
            let i = i as f32;
            let x1 = (i * self.a.x + (n - i) * self.b.x) / n;
            let y1 = (i * self.a.y + (n - i) * self.b.y) / n;
            outer_segment.push(pt2(x1, y1));

            for j in 0..=self.n.value as usize {
                let j = j as f32;

                let x2 = (j * self.c.x + (n - j) * self.d.x) / n;
                let y2 = (j * self.c.y + (n - j) * self.d.y) / n;
                inner_segment.push(pt2(x2, y2));
            }
        }

        (outer_segment, inner_segment)
    }
}

impl Default for Bipartite {
    fn default() -> Self {
        Self {
            n: F32::new_from_range(10.0, 10.0..=20.0),
            a: pt2((NP as f32) / -2.0, (NP as f32) / -2.0),
            b: pt2((NP as f32) / -2.0, (NP as f32) / 2.0),
            c: pt2((NP as f32) / 2.0, (NP as f32) / -2.0),
            d: pt2((NP as f32) / 2.0, (NP as f32) / 2.0),
        }
    }
}
