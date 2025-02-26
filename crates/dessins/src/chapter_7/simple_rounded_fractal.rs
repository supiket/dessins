use crate::{
    chapter_1::polygon::{self},
    Segment, Shape, Shapes, NP,
};
use nannou::prelude::*;
use ui_controlled_params::UiControlledParams;

pub type OuterSegment = Segment;
pub type InnerSegment = Segment;

#[derive(UiControlledParams)]
#[params(SimpleRoundedFractal)]
pub struct ParamsInner {
    #[param(range(1..=10))]
    pub m: usize, // # of segments in starting curve
    #[param(range(1..=13))]
    pub n: usize, // # of sub-segments per segment
    #[param(range(1..=10))]
    pub k: usize, // depth
    #[param(range(1..=20))]
    pub s: usize, // curve fineness
    #[param]
    pub positions: Vec<Point2>,
    #[param(range(0.0..=30.0))]
    pub lengths: Vec<f32>,
    #[param(pi)]
    pub angles: Vec<f32>,
}

impl ParamsInner {
    pub fn calculate_shapes(&mut self) -> Shapes {
        if self.positions.len() != self.m + 1 {
            self.positions
                .resize_with(Self::calculate_positions(self.m).len(), Default::default);
        }
        if self.lengths.len() != self.n {
            self.lengths.resize_with(
                Self::calculate_lengths(self.m, self.n).len(),
                Default::default,
            );
        }
        if self.angles.len() != self.n {
            self.angles.resize_with(
                Self::calculate_angles(self.m, self.n).len(),
                Default::default,
            );
        }

        let mut shapes = Shapes::default();
        let mut shape = Shape::new();

        for ii in 0..self.m {
            let mut segment = Segment::new();

            let source = self.positions[ii];
            let destination = self.positions[ii + 1];
            let diff = destination - source;

            #[allow(unused_assignments)]
            let mut point0 = source;
            let mut point1 = source;
            let mut point2 = source;

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
                point0 = point1;
                point1 = point2;
                point2 += pt2(
                    current_length * current_angle.cos(),
                    current_length * current_angle.sin(),
                );
                segment.extend(Self::curve_points(self.s, point0, point1, point2));
            }
            shape.push(segment);
        }

        shapes.push(shape);

        shapes
    }

    fn curve_points(s: usize, point0: Point2, point1: Point2, point2: Point2) -> Vec<Point2> {
        let mut points = vec![];

        let v = point1 - point0;
        let w = point2 - point1;

        for k in 0..=s {
            let angle = PI / 2.0 * k as f32 / s as f32;
            let co = angle.cos();
            let si = angle.sin();

            let q = (point0 + point2 + pt2(-co, -co) * w + pt2(si, si) * v) / pt2(2.0, 2.0);
            points.push(q);
        }

        points
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
                m: 1,
                n: 7,
                k: 2,
                s: 5,
                positions: vec![pt2(-0.5 * np, np), pt2(-0.5 * np, -np)],
                lengths: vec![0.5, 0.25, 0.25, 0.25, 0.25, 0.5, 0.5],
                angles: vec![0.0, 0.5 * PI, -PI, 0.0, 0.5 * PI, -0.5 * PI, 0.0],
            },
            calculate_shapes: Box::new(ParamsInner::calculate_shapes),
            ui_elements: Box::new(ParamsInner::ui_elements),
        }
    }
}
