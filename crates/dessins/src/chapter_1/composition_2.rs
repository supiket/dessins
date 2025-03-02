use super::{Polygon, Star};
use crate::{
    animation::AnimationState,
    meta::{FieldMeta, FieldsMeta, ParamMeta},
    shapes::{Segment, Shape, Shapes, NP},
};
use nannou::prelude::*;

#[derive(Clone, Debug, Reflect)]
#[reflect(Default)]
pub struct Composition2 {
    pub polygon: Polygon,
    pub star: Star,
    pub n: f32,  // # stars
    pub rr: f32, // reduction coefficient from one star to the next & the distance between the center of the spiral and the center of successive stars
    #[reflect(ignore)]
    pub fields_meta: Option<FieldsMeta>,
}

impl Composition2 {
    pub fn calculate_shapes(&mut self) -> Shapes {
        let mut shapes = Shapes::new();
        let mut shape = Shape::new();

        let mut polygon = self.polygon.clone();
        let mut star = self.star.clone();

        for i in 0..self.n as u32 {
            let r2 = self.polygon.r * self.rr.powi(i as i32);
            let r3 = self.star.r * self.rr.powi(i as i32);

            polygon.r = r2;
            let polygon_point = polygon.calculate_point(i);

            let mut segment = Segment::new();

            for j in 0..self.star.k as u32 {
                star.r = r3;
                let star_point = star.calculate_point(j);
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

impl ParamMeta for Composition2 {
    fn get_fields_meta(&self) -> Option<FieldsMeta> {
        if let Some(polygon_fields_meta) = &self.polygon.fields_meta {
            if let Some(star_fields_meta) = &self.star.fields_meta {
                if let Some(fields_meta) = &self.fields_meta {
                    let mut fields_meta = fields_meta.clone();
                    fields_meta.extend(polygon_fields_meta.clone());
                    fields_meta.extend(star_fields_meta.clone());
                    Some(fields_meta.clone())
                } else {
                    None
                }
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
        self.fields_meta = Some(
            [
                (format!("{}.n", path), FieldMeta::new(1.0..=100.0, 1.0)),
                (format!("{}.rr", path), FieldMeta::new(0.7..=1.3, 0.1)),
            ]
            .into_iter()
            .collect(),
        );
    }

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

impl Default for Composition2 {
    fn default() -> Self {
        Self {
            n: 32.0,
            rr: 0.9,
            polygon: Polygon {
                k: 8.0,
                r: NP as f32 * 0.36,
                ad: 0_f32,
                fields_meta: None,
            },
            star: Star {
                k: 16.0,
                h: 5.0,
                r: NP as f32 * 0.14,
                ad: 0_f32,
                fields_meta: None,
            },
            fields_meta: None,
        }
    }
}
