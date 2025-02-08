use super::{
    polygon::{self},
    star::{self},
};
use crate::{Segment, Shape, Shapes};
use nannou::prelude::*;
use nannou_egui::egui::Ui;
use ui_controlled_params::UiControlledParams;

#[derive(UiControlledParams)]
#[params(Composition2)]
pub struct ParamsInner {
    pub polygon: polygon::ParamsInner,
    pub star: star::ParamsInner,
    #[param(range(1..=100))]
    pub n: u32, // # stars
    #[param(length)]
    pub rr: f32, // reduction coefficient from one star to the next & the distance between the center of the spiral and the center of successive stars
}

pub fn calculate_shapes(inner: &ParamsInner) -> Shapes {
    let mut shapes = Shapes::new();
    let mut shape = Shape::new();

    let mut polygon = polygon::ParamsInner {
        k: inner.polygon.k,
        r: inner.polygon.r,
        ad: inner.polygon.ad,
    };
    let mut star = star::ParamsInner {
        k: inner.star.k,
        h: inner.star.h,
        r: inner.star.r,
        ad: inner.star.ad,
    };

    for i in 0..inner.n {
        let r2 = inner.polygon.r * inner.rr.powi(i as i32);
        let r3 = inner.star.r * inner.rr.powi(i as i32);

        polygon.r = r2;
        let polygon_point = polygon::calculate_point(&polygon, i);

        let mut segment = Segment::new();

        for j in 0..inner.star.k {
            star.r = r3;
            let star_point = star::calculate_point(&star, j);
            let point = star_point + polygon_point;
            segment.push(point);
        }

        segment.push(segment[0]);

        shape.push(segment);
    }

    shapes.push(shape);

    shapes
}
