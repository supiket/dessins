use crate::{
    animation::AnimationState,
    meta::{FieldMeta, FieldsMeta, ParamMeta},
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
    #[reflect(ignore)]
    pub fields_meta: Option<FieldsMeta>,
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

impl ParamMeta for Jolygon {
    fn get_fields_meta(&self) -> Option<FieldsMeta> {
        self.fields_meta.clone()
    }

    fn set_fields_meta(&mut self, path: &str) {
        self.fields_meta = Some(
            [
                (format!("{}.k", path), FieldMeta::new(1.0..=2500.0, 100.0)),
                (format!("{}.an", path), FieldMeta::new_angle()),
                (format!("{}.ra", path), FieldMeta::new(0.9..=1.0, 0.01)),
                (format!("{}.aa", path), FieldMeta::new_angle()),
                (format!("{}.rr", path), FieldMeta::new_length()),
            ]
            .into_iter()
            .collect(),
        );
    }

    fn toggle_field_animation_state(&mut self, field_key: &str) -> anyhow::Result<()> {
        if let Some(fields_meta) = &mut self.fields_meta {
            if let Some(FieldMeta { animation, subtype }) = fields_meta.get_mut(field_key) {
                *animation = match animation {
                    Some(_) => None,
                    None => {
                        // TODO: change
                        let freq = 1.0;
                        let range = subtype.get_range();
                        Some(AnimationState::new(freq, *range.start(), *range.end()))
                    }
                }
            } else {
                return Err(anyhow::anyhow!(format!(
                    "fields_meta entry at key {} is none",
                    field_key
                )));
            }
        } else {
            return Err(anyhow::anyhow!("fields_meta is none"));
        }

        Ok(())
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
            fields_meta: None,
        }
    }
}
