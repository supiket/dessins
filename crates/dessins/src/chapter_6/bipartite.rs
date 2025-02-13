use crate::{Segment, Shape, Shapes, NP};
use nannou::prelude::*;
use ui_controlled_params::UiControlledParams;

pub type OuterSegment = Segment;
pub type InnerSegment = Segment;

#[derive(UiControlledParams)]
#[params(Bipartite)]
pub struct ParamsInner {
    #[param(range(10..=20))]
    pub n: u32,
    #[param]
    pub a: Point2,
    #[param]
    pub b: Point2,
    #[param]
    pub c: Point2,
    #[param]
    pub d: Point2,
}

pub struct YParams {
    pub i: f32,
    pub n: f32,
    pub k2: f32,
}

impl ParamsInner {
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

        let n = self.n as f32;

        for i in 0..=self.n {
            let i = i as f32;
            let x1 = (i * self.a.x + (n - i) * self.b.x) / n;
            let y1 = (i * self.a.y + (n - i) * self.b.y) / n;
            outer_segment.push(pt2(x1, y1));

            for j in 0..=self.n {
                let j = j as f32;

                let x2 = (j * self.c.x + (n - j) * self.d.x) / n;
                let y2 = (j * self.c.y + (n - j) * self.d.y) / n;
                inner_segment.push(pt2(x2, y2));
            }
        }

        (outer_segment, inner_segment)
    }
}

impl Default for Params {
    fn default() -> Self {
        Self {
            inner: ParamsInner {
                n: 10,
                a: pt2((NP as f32) / -2.0, (NP as f32) / -2.0),
                b: pt2((NP as f32) / -2.0, (NP as f32) / 2.0),
                c: pt2((NP as f32) / 2.0, (NP as f32) / -2.0),
                d: pt2((NP as f32) / 2.0, (NP as f32) / 2.0),
            },
            calculate_shapes: Box::new(ParamsInner::calculate_shapes),
            ui_elements: Box::new(ParamsInner::ui_elements),
        }
    }
}
