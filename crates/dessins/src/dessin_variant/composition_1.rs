use super::{Polygon, Star};
use crate::{
    adjustable_dessin::AdjustableDessin,
    adjustable_variable::types::{
        f32::{F32Variant, F32},
        u32::U32,
    },
    shapes::{Segment, Shape, Shapes},
};
use adjustable_dessin_derive::DefaultAdjustableDessin;
use nannou::prelude::*;

#[derive(Clone, Debug, PartialEq, Reflect, DefaultAdjustableDessin)]
#[reflect(Default)]
pub struct Composition1 {
    pub polygon_k: U32,
    pub polygon_r: F32,
    pub polygon_ad: F32,
    pub star_k: U32,
    pub star_h: U32,
    pub star_r: F32,
    pub star_ad: F32,
}

impl Composition1 {
    pub fn calculate_shapes(&mut self) -> Shapes {
        let mut shapes = Shapes::new();
        let mut shape = Shape::new();

        let polygon = Polygon {
            k: self.polygon_k.clone(),
            r: self.polygon_r.clone(),
            ad: self.polygon_ad.clone(),
        };

        let star = Star {
            k: self.star_k.clone(),
            h: self.star_h.clone(),
            r: self.star_r.clone(),
            ad: self.star_ad.clone(),
        };

        for i in 0..polygon.k.get_value() {
            let polygon_point = polygon.calculate_point(i);

            let mut segment = Segment::new();

            for j in 0..star.k.get_value() {
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

impl Default for Composition1 {
    fn default() -> Self {
        Self {
            polygon_k: U32::new(5, 3..=20, 1),
            polygon_r: F32::new(0.27, F32Variant::Length),
            polygon_ad: F32::new(0.5, F32Variant::Angle),
            star_k: U32::new(25, 5..=100, 1),
            star_h: U32::new(12, 3..=5, 1),
            star_r: F32::new(0.22, F32Variant::Length),
            star_ad: F32::new(0.5, F32Variant::Angle),
        }
    }
}
