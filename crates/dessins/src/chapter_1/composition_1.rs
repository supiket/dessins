use super::{
    polygon::{self},
    star::{self},
};
use crate::{Segment, Shape, Shapes, NP};
use nannou::prelude::*;
use ui_controlled_params::UiControlledParams;

#[derive(UiControlledParams)]
#[params(Composition1)]
pub struct ParamsInner {
    #[param]
    pub polygon: polygon::ParamsInner,
    #[param]
    pub star: star::ParamsInner,
}

impl ParamsInner {
    pub fn calculate_shapes(&mut self) -> Shapes {
        let mut shapes = Shapes::default();
        let mut shape = Shape::new();

        for i in 0..self.polygon.k {
            let polygon_point = self.polygon.calculate_point(i);

            let mut segment = Segment::new();

            for j in 0..self.star.k {
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

impl Default for Params {
    fn default() -> Self {
        Self {
            inner: ParamsInner {
                polygon: polygon::ParamsInner {
                    k: 5,
                    r: NP as f32 * 0.27,
                    ad: PI / 2.0,
                },
                star: star::ParamsInner {
                    k: 25,
                    h: 12,
                    r: NP as f32 * 0.22,
                    ad: PI / 2.0,
                },
            },
            calculate_shapes: Box::new(ParamsInner::calculate_shapes),
            ui_elements: Box::new(ParamsInner::ui_elements),
        }
    }
}
