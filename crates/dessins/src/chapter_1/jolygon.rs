use crate::{
    meta::{ParamMeta, ParamsMeta},
    reflect::ControllableParams,
    shapes::{Segment, Shape, Shapes, NP},
};
use nannou::prelude::*;

#[derive(Clone, Debug, Reflect)]
#[reflect(Default)]
pub struct Jolygon {
    pub k: f32,  // # segments
    pub an: f32, // angle of two consecutive segments
    pub ra: f32, // ratio of the lengths of two consecutive segments
    pub aa: f32, // angle of the first segment with horizontal
    pub rr: f32, // length of the first segment
    pub meta: Option<ParamsMeta>,
}

impl Jolygon {
    pub fn calculate_shapes(&mut self) -> Shapes {
        let mut shapes = Shapes::new();
        let mut shape = Shape::new();
        let mut segment = Segment::new();

        let mut current_length = self.rr;
        let mut current_pos = pt2(0.0, 0.0);
        segment.push(current_pos);

        let mut min_x = 0.0;
        let mut max_x = 0.0;
        let mut min_y = 0.0;
        let mut max_y = 0.0;

        for i in 0..self.k as u32 {
            let angle = self.aa + i as f32 * self.an;

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
            current_length *= self.ra;
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

impl ControllableParams for Jolygon {
    fn set_meta(&mut self, path: &str) {
        self.meta = Some(ParamsMeta(
            [
                (format!("{}.k", path), ParamMeta::new(1.0..=2500.0)),
                (format!("{}.an", path), ParamMeta::new_angle()),
                (format!("{}.ra", path), ParamMeta::new(0.9..=1.0)),
                (format!("{}.aa", path), ParamMeta::new_angle()),
                (format!("{}.rr", path), ParamMeta::new_length()),
            ]
            .into_iter()
            .collect(),
        ));
    }
}

impl Default for Jolygon {
    fn default() -> Self {
        Self {
            k: 200.0,
            an: 15.0 * PI / 31.0,
            ra: 0.98,
            aa: 0_f32,
            rr: 0.80 * NP as f32,
            meta: None,
        }
    }
}
