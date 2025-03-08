use crate::{
    adjustable_dessin::AdjustableDessin,
    adjustable_variable::types::f32::F32,
    chapter_1::Polygon,
    shapes::{Segment, Shape, Shapes, NP},
};
use nannou::prelude::*;

pub type OuterSegment = Segment;
pub type InnerSegment = Segment;

#[derive(Clone, Debug, PartialEq, Reflect)]
#[reflect(Default)]
pub struct SimpleRoundedFractal {
    pub m: F32, // # of segments in starting curve
    pub n: F32, // # of sub-segments per segment
    pub k: F32, // depth
    pub s: F32, // curve fineness
    #[reflect(ignore)]
    pub positions: Vec<Point2>,
    #[reflect(ignore)]
    pub lengths: Vec<f32>,
    #[reflect(ignore)]
    pub angles: Vec<f32>,
}

impl SimpleRoundedFractal {
    pub fn calculate_shapes(&mut self) -> Shapes {
        if self.positions.len() != self.m.value as usize + 1 {
            self.positions
                .resize_with(Self::calculate_positions(&self.m).len(), Default::default);
        }
        if self.lengths.len() != self.n.value as usize {
            self.lengths.resize_with(
                Self::calculate_lengths(self.m.value, self.n.value as usize).len(),
                Default::default,
            );
        }
        if self.angles.len() != self.n.value as usize {
            self.angles.resize_with(
                Self::calculate_angles(self.m.value, self.n.value as usize).len(),
                Default::default,
            );
        }

        let mut shapes = Shapes::new();
        let mut shape = Shape::new();

        for ii in 0..self.m.value as usize {
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

            for i in 0..(self.n.value as usize).pow(self.k.value as u32) {
                let mut current_length = length;
                let mut current_angle = angle;
                let mut t1 = i;
                if self.k.value as usize != 0 {
                    for j in (0..self.k.value as usize).rev() {
                        let r = (self.n.value as usize).pow(j as u32);
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
                segment.extend(Self::curve_points(
                    self.s.value as usize,
                    point0,
                    point1,
                    point2,
                ));
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

    fn calculate_positions(m: &F32) -> Vec<Point2> {
        let mut polygon = Polygon {
            k: m.clone(),
            ..Default::default()
        };
        polygon.r.value = NP as f32 * 0.5;
        polygon.ad.value = 0.0;

        let mut points = vec![];
        for i in 0..m.value as usize {
            let point = polygon.calculate_point(i as u32);
            points.push(point);
        }
        points.push(points[0]);
        points
    }

    fn calculate_lengths(m: f32, n: usize) -> Vec<f32> {
        vec![1.0 / m; n]
    }

    fn calculate_angles(m: f32, n: usize) -> Vec<f32> {
        let mut angles = vec![0.0];

        for i in 1..(n - 1) {
            angles.push((PI / m) * if i % 2 == 1 { 1.0 } else { -1.0 });
        }

        angles.push(0.0);

        angles
    }
}

impl AdjustableDessin for SimpleRoundedFractal {}

impl Default for SimpleRoundedFractal {
    fn default() -> Self {
        let np = NP as f32;
        Self {
            m: F32::new_from_range(1.0, 1.0..=10.0),
            n: F32::new_from_range(7.0, 1.0..=13.0),
            k: F32::new_from_range(2.0, 1.0..=10.0),
            s: F32::new_from_range(5.0, 1.0..=20.0),
            positions: vec![pt2(-0.5 * np, np), pt2(-0.5 * np, -np)],
            lengths: vec![0.5, 0.25, 0.25, 0.25, 0.25, 0.5, 0.5],
            angles: vec![0.0, 0.5 * PI, -PI, 0.0, 0.5 * PI, -0.5 * PI, 0.0],
        }
    }
}
