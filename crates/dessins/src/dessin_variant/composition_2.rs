use super::{Polygon, Star};
use crate::{
    adjustable_dessin::AdjustableDessin,
    adjustable_variable::types::f32::{F32Variant, F32},
    shapes::{Segment, Shape, Shapes},
};
use adjustable_dessin_derive::DefaultAdjustableDessin;
use nannou::prelude::*;

#[derive(Clone, Debug, PartialEq, Reflect, DefaultAdjustableDessin)]
#[reflect(Default)]
pub struct Composition2 {
    pub polygon_k: F32,
    pub polygon_r: F32,
    pub polygon_ad: F32,
    pub star_k: F32,
    pub star_h: F32,
    pub star_r: F32,
    pub star_ad: F32,
    pub n: F32,  // # stars
    pub rr: F32, // reduction coefficient from one star to the next & the distance between the center of the spiral and the center of successive stars
}

impl Composition2 {
    pub fn calculate_shapes(&mut self) -> Shapes {
        let mut shapes = Shapes::new();
        let mut shape = Shape::new();

        let mut polygon = Polygon {
            k: self.polygon_k.clone(),
            r: self.polygon_r.clone(),
            ad: self.polygon_ad.clone(),
        };

        let mut star = Star {
            k: self.star_k.clone(),
            h: self.star_h.clone(),
            r: self.star_r.clone(),
            ad: self.star_ad.clone(),
        };

        for i in 0..self.n.value as u32 {
            let r2 = polygon.r.value * self.rr.value.powi(i as i32);
            let r3 = star.r.value * self.rr.value.powi(i as i32);

            polygon.r.value = r2;
            let polygon_point = polygon.calculate_point(i);

            let mut segment = Segment::new();

            for j in 0..star.k.value as u32 {
                star.r.value = r3;
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

impl Default for Composition2 {
    fn default() -> Self {
        Self {
            polygon_k: F32::new_from_range(5.0, 3.0..=20.0),
            polygon_r: F32::new(0.27, F32Variant::Length),
            polygon_ad: F32::new(0.5, F32Variant::Angle),
            star_k: F32::new_from_range(25.0, 5.0..=100.0),
            star_h: F32::new_from_range(12.0, 3.0..=5.0),
            star_r: F32::new(0.22, F32Variant::Length),
            star_ad: F32::new(0.5, F32Variant::Angle),
            n: F32::new_from_range(32.0, 1.0..=100.0),
            rr: F32::new_from_range(0.9, 0.7..=1.3),
        }
    }
}
