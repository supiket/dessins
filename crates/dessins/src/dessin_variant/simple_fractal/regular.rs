use crate::{
    adjustable_dessin::AdjustableDessin,
    adjustable_variable::types::{F32Variant, Pt2, VecF32, VecPt2, F32, U32},
    dessin_variant::Polygon,
    shapes::{sign, Segment, Shape, Shapes},
};
use adjustable_dessin_derive::DefaultAdjustableDessin;
use nannou::prelude::*;

#[derive(Clone, Debug, PartialEq, Reflect, DefaultAdjustableDessin)]
#[reflect(Default)]
pub struct Regular {
    pub m: U32, // # of segments in starting curve
    pub n: U32, // # of sub-segments per segment
    pub k: U32, // depth
    pub positions: VecPt2,
    pub lengths: VecF32,
    pub angles: VecF32,
}

impl Regular {
    pub fn calculate_shapes(&mut self) -> Shapes {
        if self.positions.get_value().len() != self.m.get_value() as usize + 1 {
            let positions = Self::calculate_positions(&self.m)
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
            let angles =
                Self::calculate_angles(self.m.get_value() as f32, self.n.get_value() as usize)
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

            let mut point = source;
            segment.push(point);

            let angle = if diff.x == 0.0 {
                PI / 2.0 * sign(diff.y)
            } else {
                (diff.y / diff.x).atan()
            } + if diff.x < 0.0 { PI } else { 0.0 };

            let length = diff.length();

            for i in 0..(self.n.get_value() as usize).pow(self.k.get_value()) {
                let mut current_length = length;
                let mut current_angle = angle;
                let mut t1 = i;
                if self.k.get_value() != 0 {
                    for j in (0..self.k.get_value()).rev() {
                        let r = (self.n.get_value() as usize).pow(j);
                        let t2 = t1 / r;
                        current_angle += self.angles.get_value()[t2].get_value();
                        current_length *= self.lengths.get_value()[t2].get_value();
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

    pub fn calculate_positions(m: &U32) -> Vec<Point2> {
        let polygon = Polygon {
            k: m.clone(),
            r: F32::new(0.58, F32Variant::Length),
            ad: F32::new(-7.0 / 6.0, F32Variant::Angle),
        };

        let mut points = vec![];
        for i in 0..m.get_value() as usize {
            let point = polygon.calculate_point(i as u32);
            points.push(point);
        }
        points.push(points[0]);

        points.into_iter().rev().collect()
    }

    pub fn calculate_lengths(m: f32, n: usize) -> Vec<f32> {
        vec![1.0 / m; n]
    }

    pub fn calculate_angles(m: f32, n: usize) -> Vec<f32> {
        let mut angles = vec![0.0];

        for i in 1..(n - 1) {
            angles.push((1.0 / m) * if i % 2 == 1 { 1.0 } else { -1.0 });
        }

        angles.push(0.0);

        angles
    }
}

impl Default for Regular {
    fn default() -> Self {
        let m = U32::new(3, 1..=4);
        let n = 4;
        let positions = Self::calculate_positions(&m);
        let angles = Self::calculate_angles(m.get_value() as f32, n);
        let lengths = Self::calculate_lengths(m.get_value() as f32, n);
        Self {
            m,
            n: U32::new(n as u32, 2..=5),
            k: U32::new(4, 1..=6),
            positions: VecPt2::new(positions),
            lengths: VecF32::new(lengths, F32Variant::None(0.0..=1.0)),
            angles: VecF32::new(angles, F32Variant::Angle),
        }
    }
}
