use super::{Polygon, Star};
use crate::{
    meta::{ParamMeta, ParamsMeta},
    reflect::ControllableParams,
    shapes::{Segment, Shape, Shapes, NP},
};
use nannou::prelude::*;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Reflect)]
#[reflect(Default)]
pub struct Composition2 {
    pub polygon_k: f32,
    pub polygon_r: f32,
    pub polygon_ad: f32,
    pub star_k: f32,
    pub star_h: f32,
    pub star_r: f32,
    pub star_ad: f32,
    pub n: f32,  // # stars
    pub rr: f32, // reduction coefficient from one star to the next & the distance between the center of the spiral and the center of successive stars
    pub meta: Option<ParamsMeta>,
}

impl Composition2 {
    pub fn calculate_shapes(&mut self) -> Shapes {
        let mut shapes = Shapes::new();
        let mut shape = Shape::new();

        let mut polygon = Polygon {
            k: self.polygon_k,
            r: self.polygon_r,
            ad: self.polygon_ad,
            meta: None,
        };

        let mut star = Star {
            k: self.star_k,
            h: self.star_h,
            r: self.star_r,
            ad: self.star_ad,
            meta: None,
        };

        for i in 0..self.n as u32 {
            let r2 = polygon.r * self.rr.powi(i as i32);
            let r3 = star.r * self.rr.powi(i as i32);

            polygon.r = r2;
            let polygon_point = polygon.calculate_point(i);

            let mut segment = Segment::new();

            for j in 0..star.k as u32 {
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

impl ControllableParams for Composition2 {
    fn set_meta(&mut self, path: &str) {
        let mut polygon = Polygon {
            k: self.polygon_k,
            r: self.polygon_r,
            ad: self.polygon_ad,
            meta: None,
        };

        let mut star = Star {
            k: self.star_k,
            h: self.star_h,
            r: self.star_r,
            ad: self.star_ad,
            meta: None,
        };

        polygon.set_meta(format!("{}.polygon", path).as_str());
        star.set_meta(format!("{}.star", path).as_str());

        let mut meta = ParamsMeta(HashMap::new());

        for (k, v) in polygon.get_meta().expect("set just now").iter() {
            let new_k = k.replace("polygon.", "polygon_");
            meta.insert(new_k, v.clone());
        }

        for (k, v) in star.get_meta().expect("set just now").iter() {
            let new_k = k.replace("star.", "star_");
            meta.insert(new_k, v.clone());
        }

        meta.insert(
            format!("{}.n", path),
            ParamMeta::new_f32_from_range(1.0..=100.0),
        );
        meta.insert(
            format!("{}.rr", path),
            ParamMeta::new_f32_from_range(0.7..=1.3),
        );

        self.meta = Some(meta);
    }
}

impl Default for Composition2 {
    fn default() -> Self {
        Self {
            polygon_k: 5.0,
            polygon_r: NP as f32 * 0.27,
            polygon_ad: PI / 2.0,

            star_k: 25.0,
            star_h: 12.0,
            star_r: NP as f32 * 0.22,
            star_ad: PI / 2.0,

            n: 32.0,
            rr: 0.9,

            meta: None,
        }
    }
}
