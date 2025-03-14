use crate::{
    adjustable_dessin::AdjustableDessin,
    adjustable_variable::types::u32::U32,
    dessin_variant::Polygon,
    shapes::{sign, Segment, Shape, Shapes, NP},
};
use adjustable_dessin_derive::DefaultAdjustableDessin;
use nannou::prelude::*;

#[derive(Clone, Debug, PartialEq, Reflect, DefaultAdjustableDessin)]
#[reflect(Default)]
pub struct Regular {
    pub m: U32, // # of segments in starting curve
    pub n: U32, // # of sub-segments per segment
    pub k: U32, // depth
    #[reflect(ignore)]
    pub positions: Vec<Point2>,
    #[reflect(ignore)]
    pub lengths: Vec<f32>,
    #[reflect(ignore)]
    pub angles: Vec<f32>,
}

impl Regular {
    pub fn calculate_shapes(&mut self) -> Shapes {
        if self.positions.len() != self.m.get_value() as usize + 1 {
            self.positions
                .resize_with(Self::calculate_positions(&self.m).len(), Default::default);
        }
        if self.lengths.len() != self.n.get_value() as usize {
            self.lengths.resize_with(
                Self::calculate_lengths(self.m.get_value() as f32, self.n.get_value() as usize)
                    .len(),
                Default::default,
            );
        }
        if self.angles.len() != self.n.get_value() as usize {
            self.angles.resize_with(
                Self::calculate_angles(self.m.get_value() as f32, self.n.get_value() as usize)
                    .len(),
                Default::default,
            );
        }

        let mut shapes = Shapes::new();
        let mut shape = Shape::new();

        for ii in 0..self.m.get_value() as usize {
            let mut segment = Segment::new();

            let source = self.positions[ii];
            let destination = self.positions[ii + 1];
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

    fn calculate_positions(m: &U32) -> Vec<Point2> {
        let mut polygon = Polygon {
            k: m.clone(),
            ..Default::default()
        };
        polygon.r.set_value(NP as f32 * 0.5);
        polygon.ad.set_value(0.0);

        let mut points = vec![];
        for i in 0..m.get_value() as usize {
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

impl Default for Regular {
    fn default() -> Self {
        let np = NP as f32;
        let y0 = (f32::sqrt(3.0) / 2.0 - 0.5) * np;
        Self {
            m: U32::new(3, 1..=10),
            n: U32::new(4, 1..=10),
            k: U32::new(4, 1..=10),
            positions: vec![
                pt2(-0.5 * np, y0),
                pt2(0.5 * np, y0),
                pt2(0.0 * np, -0.5 * np),
                pt2(-0.5 * np, y0),
            ],
            lengths: vec![1.0 / 3.0; 4],
            angles: vec![0.0, PI / 3.0, -PI / 3.0, 0.0],
        }
    }
}
