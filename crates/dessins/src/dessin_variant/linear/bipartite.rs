use crate::{
    adjustable_dessin::AdjustableDessin,
    adjustable_variable::types::{pt2::Pt2, u32::U32},
    shapes::{Segment, Shape, Shapes, NP},
};
use nannou::prelude::*;

pub type OuterSegment = Segment;
pub type InnerSegment = Segment;
use adjustable_dessin_derive::DefaultAdjustableDessin;

#[derive(Clone, Debug, PartialEq, Reflect, DefaultAdjustableDessin)]
#[reflect(Default)]
pub struct Bipartite {
    pub n: U32,
    pub a: Pt2,
    pub b: Pt2,
    pub c: Pt2,
    pub d: Pt2,
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

        let n = self.n.get_value() as f32;
        let a = self.a.get_value();
        let b = self.b.get_value();
        let c = self.c.get_value();
        let d = self.d.get_value();

        for i in 0..=self.n.get_value() {
            let i = i as f32;
            let x1 = (i * a.x + (n - i) * b.x) / n;
            let y1 = (i * a.y + (n - i) * b.y) / n;
            outer_segment.push(pt2(x1, y1));

            for j in 0..=self.n.get_value() {
                let j = j as f32;

                let x2 = (j * c.x + (n - j) * d.x) / n;
                let y2 = (j * c.y + (n - j) * d.y) / n;
                inner_segment.push(pt2(x2, y2));
            }
        }

        (outer_segment, inner_segment)
    }
}

impl Default for Bipartite {
    fn default() -> Self {
        Self {
            n: U32::new(10, 10..=20, 1),
            a: Pt2::new(pt2((NP as f32) / -2.0, (NP as f32) / -2.0)),
            b: Pt2::new(pt2((NP as f32) / -2.0, (NP as f32) / 2.0)),
            c: Pt2::new(pt2((NP as f32) / 2.0, (NP as f32) / -2.0)),
            d: Pt2::new(pt2((NP as f32) / 2.0, (NP as f32) / 2.0)),
        }
    }
}
