use crate::{
    adjustable_dessin::AdjustableDessin,
    adjustable_variable::types::f32::{F32Variant, F32},
    shapes::{Segment, Shape, Shapes},
};
use adjustable_dessin_derive::DefaultAdjustableDessin;
use nannou::prelude::*;

#[derive(Clone, Debug, PartialEq, Reflect, DefaultAdjustableDessin)]
#[reflect(Default)]
pub struct Jolygon {
    pub k: F32,  // # segments
    pub an: F32, // angle of two consecutive segments
    pub ra: F32, // ratio of the lengths of two consecutive segments
    pub aa: F32, // angle of the first segment with horizontal
    pub rr: F32, // length of the first segment
}

impl Jolygon {
    pub fn calculate_shapes(&mut self) -> Shapes {
        let mut shapes = Shapes::new();
        let mut shape = Shape::new();
        let mut segment = Segment::new();

        let mut current_length = self.rr.value;
        let mut current_pos = pt2(0.0, 0.0);
        segment.push(current_pos);

        let mut min_x: f32 = 0.0;
        let mut max_x: f32 = 0.0;
        let mut min_y: f32 = 0.0;
        let mut max_y: f32 = 0.0;

        for i in 0..self.k.value as u32 {
            let angle = self.aa.value + i as f32 * self.an.value;

            let dx = current_length * angle.cos();
            let dy = current_length * angle.sin();
            let d = pt2(dx, dy);
            let point = current_pos + d;

            // update bounds
            min_x = min_x.min(point.x);
            max_x = max_x.max(point.x);
            min_y = min_y.min(point.y);
            max_y = max_y.max(point.y);

            segment.push(point);
            current_pos = point;
            current_length *= self.ra.value;
        }

        // calculate center offset
        let center_offset_x = (min_x + max_x) / 2.0;
        let center_offset_y = (min_y + max_y) / 2.0;

        // make segments centered
        for point in &mut segment {
            *point -= pt2(center_offset_x, center_offset_y);
        }

        shape.push(segment);
        shapes.push(shape);

        shapes
    }
}

impl Default for Jolygon {
    fn default() -> Self {
        Self {
            k: F32::new_from_range(200.0, 1.0..=2500.0),
            an: F32::new(15.0 / 31.0, F32Variant::Angle),
            ra: F32::new_from_range(0.98, 0.9..=1.0),
            aa: F32::new(0.0, F32Variant::Angle),
            rr: F32::new(0.8, F32Variant::Length),
        }
    }
}
