use super::{
    polygon::{self},
    star::{self},
};
use crate::{Segment, Shape, Shapes, NP};
use nannou::prelude::*;
use ui_controlled_params::UiControlledParams;

#[derive(UiControlledParams)]
#[params(Composition2)]
pub struct ParamsInner {
    #[param]
    pub polygon: polygon::ParamsInner,
    #[param]
    pub star: star::ParamsInner,
    #[param(range(1..=100))]
    pub n: u32, // # stars
    #[param(range(0.7..=1.3))]
    pub rr: f32, // reduction coefficient from one star to the next & the distance between the center of the spiral and the center of successive stars
}

impl ParamsInner {
    pub fn calculate_shapes(&mut self) -> Shapes {
        let mut shapes = Shapes::new();
        let mut shape = Shape::new();

        let mut polygon = polygon::ParamsInner {
            k: self.polygon.k,
            r: self.polygon.r,
            ad: self.polygon.ad,
        };
        let mut star = star::ParamsInner {
            k: self.star.k,
            h: self.star.h,
            r: self.star.r,
            ad: self.star.ad,
        };

        for i in 0..self.n {
            let r2 = self.polygon.r * self.rr.powi(i as i32);
            let r3 = self.star.r * self.rr.powi(i as i32);

            polygon.r = r2;
            let polygon_point = polygon.calculate_point(i);

            let mut segment = Segment::new();

            for j in 0..self.star.k {
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

impl Default for Params {
    fn default() -> Self {
        Self {
            inner: ParamsInner {
                n: 32,
                rr: 0.9,
                polygon: polygon::ParamsInner {
                    k: 8,
                    r: NP as f32 * 0.36,
                    ad: 0_f32,
                },
                star: star::ParamsInner {
                    k: 16,
                    h: 5,
                    r: NP as f32 * 0.14,
                    ad: 0_f32,
                },
            },
            calculate_shapes: Box::new(ParamsInner::calculate_shapes),
            ui_elements: Box::new(ParamsInner::ui_elements),
        }
    }
}
