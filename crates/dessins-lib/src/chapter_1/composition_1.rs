use super::{
    polygon::{self},
    star::{self},
};
use crate::{Segment, Shape, Shapes};
use nannou::prelude::*;
use nannou_egui::egui::Ui;
use ui_controlled_params::UiControlledParams;

#[derive(UiControlledParams)]
#[params(Composition1)]
pub struct ParamsInner {
    #[param]
    pub polygon: polygon::ParamsInner,
    #[param]
    pub star: star::ParamsInner,
}

pub fn calculate_shapes(inner: &ParamsInner) -> Shapes {
    let mut shapes = Shapes::new();
    let mut shape = Shape::new();

    for i in 0..inner.polygon.k {
        let polygon_point = polygon::calculate_point(&inner.polygon, i);

        let mut segment = Segment::new();

        for j in 0..inner.star.k {
            let star_point = star::calculate_point(&inner.star, j);
            let point = star_point + polygon_point;
            segment.push(point);
        }

        segment.push(segment[0]);

        shape.push(segment);
    }

    shapes.push(shape);
    shapes
}
