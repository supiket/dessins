use super::{
    polygon::{self},
    star::{self},
};
use crate::{add_float_slider, add_number_slider, DesignParams, Model, Segment, Shape, Shapes};
use nannou::prelude::*;
use nannou_egui::egui::Ui;

pub struct ParamsInner {
    pub polygon: polygon::ParamsInner,
    pub star: star::ParamsInner,
    pub n: u32,  // # stars
    pub rr: f32, // reduction coefficient from one star to the next & the distance between the center of the spiral and the center of successive stars
}

pub struct Params {
    pub inner: ParamsInner,
    pub calculate_shapes: Box<dyn Fn(&ParamsInner) -> Shapes>,
    pub ui_elements: Box<dyn Fn(&mut ParamsInner, &mut Ui) -> bool>,
}

pub fn model(app: &App, inner: ParamsInner) -> Model {
    let params = DesignParams::Composition2(Params {
        inner,
        calculate_shapes: Box::new(calculate_shapes),
        ui_elements: Box::new(ui_elements),
    });

    crate::model(params, app)
}

pub fn calculate_shapes(inner: &ParamsInner) -> Shapes {
    let mut shapes = Shapes::new();
    let mut shape = Shape::new();

    for i in 0..inner.n {
        let r2 = inner.polygon.r * inner.rr.powi(i as i32);
        let r3 = inner.star.r * inner.rr.powi(i as i32);

        let mut polygon_inner = inner.polygon.clone();
        polygon_inner.r = r2;
        let polygon_point = polygon::calculate_point(&polygon_inner, i);

        let mut segment = Segment::new();

        for j in 0..inner.star.k {
            let mut star_inner = inner.star.clone();
            star_inner.r = r3;
            let star_point = star::calculate_point(&star_inner, j);
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
    add_number_slider(ui, "n", &mut inner.n, 1..=100)
        || add_float_slider(ui, "rr", &mut inner.rr, 0.0..=1.0)
        || polygon::ui_elements(&mut inner.polygon, ui)
        || star::ui_elements(&mut inner.star, ui)
}
