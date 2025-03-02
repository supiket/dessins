use crate::{
    animation::AnimationState,
    meta::{FieldMeta, FieldsMeta, ParamMeta},
    shapes::{Segment, Shape, Shapes, NP},
};
use nannou::prelude::*;

#[derive(Clone, Debug, Reflect)]
#[reflect(Default)]
pub struct Polygon {
    // TODO: reconsider f32, at least discretize behaviour
    pub k: f32,  // # vertices
    pub r: f32,  // radius of the circle on which the vertices are
    pub ad: f32, // angle (in radians) of the vector CS with horizontal, where S is the first vertex,
    #[reflect(ignore)]
    pub fields_meta: Option<FieldsMeta>,
}

impl Polygon {
    pub fn calculate_shapes(&mut self) -> Shapes {
        let mut shapes = Shapes::new();
        let mut shape = Shape::new();
        let mut segment = Segment::new();

        for i in 0..self.k as u32 {
            let point = self.calculate_point(i);
            segment.push(point);
        }

        segment.push(segment[0]);

        shape.push(segment);
        shapes.push(shape);
        shapes
    }

    pub fn calculate_point(&self, i: u32) -> Point2 {
        let angle = (2.0 * i as f32 * PI) / self.k as f32 + self.ad;
        let x = self.r * angle.cos();
        let y = self.r * angle.sin();
        pt2(x, y)
    }
}

impl ParamMeta for Polygon {
    fn get_fields_meta(&self) -> Option<FieldsMeta> {
        self.fields_meta.clone()
    }

    fn set_fields_meta(&mut self, path: &str) {
        self.fields_meta = Some(
            [
                (format!("{}.k", path), FieldMeta::new(3.0..=20.0, 1.0)),
                (format!("{}.r", path), FieldMeta::new_length()),
                (format!("{}.ad", path), FieldMeta::new_angle()),
            ]
            .into_iter()
            .collect(),
        );
    }

    // TODO: generate from reflect
    fn toggle_field_animation_state(&mut self, field_key: &str) -> anyhow::Result<()> {
        if let Some(fields_meta) = &mut self.fields_meta {
            if let Some(FieldMeta { animation, subtype }) = fields_meta.get_mut(field_key) {
                *animation = match animation {
                    Some(_) => None,
                    None => {
                        // TODO: add to FieldMeta
                        let freq = 1.0;
                        let range = subtype.get_range();
                        Some(AnimationState::new(freq, *range.start(), *range.end()))
                    }
                };
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

impl Default for Polygon {
    fn default() -> Self {
        Self {
            k: 3.0,
            r: NP as f32 * 0.45,
            ad: 0_f32,
            fields_meta: None,
        }
    }
}
