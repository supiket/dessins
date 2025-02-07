use super::{
    polygon::{self},
    star::{self},
};
use crate::{DesignParams, Model, Segment, Shape, Shapes};
use nannou::prelude::*;
use nannou_egui::egui::Ui;

pub struct ParamsInner {
    pub polygon: polygon::ParamsInner,
    pub star: star::ParamsInner,
}

pub struct Params {
    pub inner: ParamsInner,
    pub calculate_shapes: Box<dyn Fn(&ParamsInner) -> Shapes>,
    pub ui_elements: Box<dyn Fn(&mut ParamsInner, &mut Ui) -> bool>,
}

pub fn model(app: &App, inner: ParamsInner) -> Model {
    let params = DesignParams::Composition1(Params {
        inner,
        calculate_shapes: Box::new(calculate_shapes),
        ui_elements: Box::new(ui_elements),
    });

    crate::model(params, app)
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

pub fn ui_elements(inner: &mut ParamsInner, ui: &mut Ui) -> bool {
    polygon::ui_elements(&mut inner.polygon, ui) || star::ui_elements(&mut inner.star, ui)
}
