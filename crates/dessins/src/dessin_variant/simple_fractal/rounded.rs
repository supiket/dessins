use crate::{
    adjustable_dessin::AdjustableDessin,
    adjustable_variable::types::{F32Variant, Pt2, VecF32, VecPt2, F32, U32},
    shapes::{Segment, Shape, Shapes, NP},
};
use nannou::prelude::*;

pub type OuterSegment = Segment;
pub type InnerSegment = Segment;
use adjustable_dessin_derive::DefaultAdjustableDessin;

#[derive(Clone, Debug, PartialEq, Reflect, DefaultAdjustableDessin)]
#[reflect(Default)]
pub struct Rounded {
    pub m: U32, // # of segments in starting curve
    pub n: U32, // # of sub-segments per segment
    pub k: U32, // depth
    pub s: U32, // curve fineness
    pub positions: VecPt2,
    pub lengths: VecF32,
    pub angles: VecF32,
}

impl Rounded {
    pub fn calculate_shapes(&mut self) -> Shapes {
        if self.positions.get_value().len() != self.m.get_value() as usize + 1 {
            let positions = Self::calculate_positions(self.m.get_value())
                .into_iter()
                .map(Pt2::new)
                .collect();
            self.positions.set_value(positions);
        }
        if self.lengths.get_value().len() != self.n.get_value() as usize {
            let lengths =
                Self::calculate_lengths(self.m.get_value() as f32, self.n.get_value() as usize)
                    .into_iter()
                    .map(|l| F32::new(l, F32Variant::None(0.0..=1.0)))
                    .collect();
            self.lengths.set_value(lengths);
        }
        if self.angles.get_value().len() != self.n.get_value() as usize {
            let angles = Self::calculate_angles(self.n.get_value() as usize)
                .into_iter()
                .map(|a| F32::new(a, F32Variant::Angle))
                .collect();
            self.angles.set_value(angles);
        }

        let mut shapes = Shapes::new();
        let mut shape = Shape::new();

        for ii in 0..self.m.get_value() as usize {
            let mut segment = Segment::new();

            let positions = self.positions.get_value();

            let source = positions[ii].get_value();
            let destination = positions[ii + 1].get_value();
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

            for i in 0..(self.n.get_value()).pow(self.k.get_value()) {
                let mut current_length = length;
                let mut current_angle = angle;
                let mut t1 = i;
                if self.k.get_value() != 0 {
                    for j in (0..self.k.get_value()).rev() {
                        let r = (self.n.get_value()).pow(j);
                        let t2 = t1 / r;
                        current_angle += self.angles.get_value()[t2 as usize].get_value();
                        current_length *= self.lengths.get_value()[t2 as usize].get_value();
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
                    self.s.get_value() as usize,
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

    fn calculate_positions(m: u32) -> Vec<Point2> {
        let np = NP as f32;
        let mut positions = vec![pt2(-0.5 * np, 0.5 * np), pt2(0.5 * np, -0.5 * np)];
        if m == 2 {
            positions.push(positions[0]);
        }
        positions
    }

    fn calculate_lengths(m: f32, n: usize) -> Vec<f32> {
        vec![0.4 / m; n]
    }

    fn calculate_angles(n: usize) -> Vec<f32> {
        let mut angles = vec![];

        for i in 0..(n - 1) {
            angles.push(if i % 2 == 0 {
                0.0
            } else if i % 3 == 0 {
                -1.0
            } else {
                0.5
            });
        }

        angles.push(0.0);

        angles
    }
}

impl Default for Rounded {
    fn default() -> Self {
        let m = 1;
        let positions = Self::calculate_positions(m);
        Self {
            m: U32::new(m, 1..=2),
            n: U32::new(13, 4..=13),
            k: U32::new(2, 1..=5),
            s: U32::new(4, 1..=10),
            positions: VecPt2::new(positions),
            lengths: VecF32::new(
                vec![
                    0.4, 0.4, 0.2, 0.2, 0.2, 0.2, 0.4, 0.4, 0.2, 0.2, 0.2, 0.2, 0.2,
                ],
                F32Variant::None(0.0..=1.0),
            ),
            angles: VecF32::new(
                vec![
                    0.0, 0.5, 0.0, -0.5, 0.0, -0.5, -1.0, -0.5, 0.0, 0.5, 0.0, 0.5, 0.0,
                ],
                F32Variant::Angle,
            ),
        }
    }
}
