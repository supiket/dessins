use super::{Polygon, Star};
use crate::{
    animation::AnimationState,
    meta::{FieldMeta, FieldsMeta, ParamMeta},
    shapes::{Segment, Shape, Shapes, NP},
};
use nannou::prelude::*;

#[derive(Clone, Debug, Reflect)]
#[reflect(Default)]
pub struct Composition1 {
    pub polygon: Polygon,
    pub star: Star,
}

impl Composition1 {
    pub fn calculate_shapes(&mut self) -> Shapes {
        let mut shapes = Shapes::new();
        let mut shape = Shape::new();

        for i in 0..self.polygon.k as u32 {
            let polygon_point = self.polygon.calculate_point(i);

            let mut segment = Segment::new();

            for j in 0..self.star.k as u32 {
                let star_point = self.star.calculate_point(j);
                let point = star_point + polygon_point;
                segment.push(point);
            }

            segment.push(segment[0]);

            shape.push(segment);
        }

        shapes.push(shape);
        shapes
    }
}

impl ParamMeta for Composition1 {
    fn get_fields_meta(&self) -> Option<FieldsMeta> {
        if let Some(polygon_fields_meta) = &self.polygon.fields_meta {
            if let Some(star_fields_meta) = &self.star.fields_meta {
                let mut fields_meta = FieldsMeta::new();
                fields_meta.extend(polygon_fields_meta.clone());
                fields_meta.extend(star_fields_meta.clone());
                Some(fields_meta)
            } else {
                None
            }
        } else {
            None
        }
    }

    fn set_fields_meta(&mut self, path: &str) {
        self.polygon
            .set_fields_meta(format!("{}.polygon", path).as_str());
        self.star.set_fields_meta(format!("{}.star", path).as_str());
    }

    // TODO: adapt
    fn toggle_field_animation_state(&mut self, field_key: &str) -> anyhow::Result<()> {
        if let Some(fields_meta) = &mut self.get_fields_meta() {
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

impl Default for Composition1 {
    fn default() -> Self {
        Self {
            polygon: Polygon {
                k: 5.0,
                r: NP as f32 * 0.27,
                ad: PI / 2.0,
                fields_meta: None,
            },
            star: Star {
                k: 25.0,
                h: 12.0,
                r: NP as f32 * 0.22,
                ad: PI / 2.0,
                fields_meta: None,
            },
        }
    }
}
