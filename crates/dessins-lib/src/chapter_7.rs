use crate::{
    chapter_1::polygon::{self},
    Segment, Shape, Shapes, NP,
};
use nannou::prelude::*;
use ui_controlled_params::UiControlledParams;

pub type OuterSegment = Segment;
pub type InnerSegment = Segment;

#[derive(UiControlledParams)]
#[params(SimpleFractal)]
pub struct ParamsInner {
    #[param(range(1..=10))]
    pub m: usize, // # of segments in starting curve
    #[param(range(1..=10))]
    pub n: usize, // # of sub-segments per segment
    #[param(range(1..=10))]
    pub k: usize, // depth
    #[param(position)]
    pub positions: Vec<Point2>,
    #[param(length)]
    pub lengths: Vec<f32>,
    #[param(pi)]
    pub angles: Vec<f32>,
}

impl ParamsInner {
    pub fn calculate_shapes(&mut self) -> Shapes {
        if self.positions.len() != self.m + 1 {
            self.positions = Self::calculate_positions(self.m);
        }
        if self.lengths.len() != self.n {
            self.lengths = Self::calculate_lengths(self.m, self.n);
        }
        if self.angles.len() != self.n {
            self.angles = Self::calculate_angles(self.m, self.n);
        }

        let mut shapes = Shapes::new();
        let mut shape = Shape::new();

        for ii in 0..self.m {
            let mut segment = Segment::new();

            let source = self.positions[ii];
            let destination = self.positions[ii + 1];
            let diff = destination - source;

            let mut point = source;
            segment.push(point);

            let angle = if diff.x == 0.0 {
                PI / 2.0 * if diff.y < 0.0 { -1.0 } else { 1.0 }
            } else {
                (diff.y / diff.x).atan()
            } + if diff.x < 0.0 { PI } else { 0.0 };

            let length = diff.length();

            for i in 0..(self.n).pow(self.k as u32) {
                let mut current_length = length;
                let mut current_angle = angle;
                let mut t1 = i;
                if self.k != 0 {
                    for j in (0..self.k).rev() {
                        let r = (self.n).pow(j as u32);
                        let t2 = t1 / r;
                        current_angle += self.angles[t2];
                        current_length *= self.lengths[t2];
                        t1 -= t2 * r;
                    }
                }
                point += pt2(
                    current_length * current_angle.cos(),
                    current_length * current_angle.sin(),
                );
                segment.push(point);
            }
            shape.push(segment);
        }

        shapes.push(shape);

        shapes
    }

    fn calculate_positions(m: usize) -> Vec<Point2> {
        let params = polygon::ParamsInner {
            k: m as u32,
            r: NP as f32 * 0.5,
            ad: 0.0,
        };
        let mut points = vec![];
        for i in 0..m {
            let point = params.calculate_point(i as u32);
            points.push(point);
        }
        points.push(points[0]);
        points
    }

    fn calculate_lengths(m: usize, n: usize) -> Vec<f32> {
        vec![1.0 / (m as f32); n]
    }

    fn calculate_angles(m: usize, n: usize) -> Vec<f32> {
        let mut angles = vec![0.0];

        for i in 1..(n - 1) {
            angles.push((PI / (m as f32)) * if i % 2 == 1 { 1.0 } else { -1.0 });
        }

        angles.push(0.0);

        angles
    }
}

impl Default for Params {
    fn default() -> Self {
        let np = NP as f32;
        Self {
            inner: ParamsInner {
                m: 4,
                n: 4,
                k: 5,
                positions: vec![
                    pt2(-0.5 * np, -0.5 * np),
                    pt2(0.5 * np, -0.5 * np),
                    pt2(0.5 * np, 0.5 * np),
                    pt2(-0.5 * np, 0.5 * np),
                    pt2(-0.5 * np, -0.5 * np),
                ],
                lengths: vec![1.0 / (2.0 + 2.0 * (0.45 * PI).cos()); 4],
                angles: vec![0.0, 0.45 * PI, -0.45 * PI, 0.0],
            },
            calculate_shapes: Box::new(ParamsInner::calculate_shapes),
            ui_elements: Box::new(ParamsInner::ui_elements),
        }
    }
}
