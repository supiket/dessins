use crate::{
    add_float_slider, add_float_slider_np_length, add_float_slider_pi, add_number_slider,
    DesignParams, Model, Segment, Shape, Shapes,
};
use nannou::prelude::*;
use nannou_egui::egui::Ui;

pub struct ParamsInner {
    pub n: u32,
    pub k: u32,
    pub ra: f32,
    pub ll: f32,
    pub aa: f32,
    pub p0: Point2,
    pub a0: f32,
}

pub struct Params {
    pub inner: ParamsInner,
    pub calculate_shapes: Box<dyn Fn(&ParamsInner) -> Shapes>,
    pub ui_elements: Box<dyn Fn(&mut ParamsInner, &mut Ui) -> bool>,
}

pub fn model(app: &App, inner: ParamsInner) -> Model {
    let params = DesignParams::Fractal(Params {
        inner,
        calculate_shapes: Box::new(calculate_shapes),
        ui_elements: Box::new(ui_elements),
    });

    crate::model(params, app)
}

pub fn calculate_shapes(inner: &ParamsInner) -> Shapes {
    let mut shapes = Shapes::new();
    let mut shape = Shape::new();
    let mut segment = Segment::new();

    let mut p0 = inner.p0;
    let mut a0 = inner.a0;

    let nn = inner.n * (inner.n - 1).pow(inner.k - 1) - 1;

    for i in 0..=nn {
        let mut i1 = i;
        let mut h = 0;

        while i1 % (inner.n - 1) == 0 && h < (inner.k - 1) {
            i1 /= inner.n - 1;
            h += 1;
        }

        let l0 = inner.ll * inner.ra.powf((inner.k - 1 - h) as f32);
        a0 += inner.aa;

        let point = p0 + pt2(l0 * a0.cos(), l0 * a0.sin());

        segment.push(point);
        p0 = point;
    }

    shape.push(segment);
    shapes.push(shape);

    shapes
}

pub fn ui_elements(inner: &mut ParamsInner, ui: &mut Ui) -> bool {
    add_number_slider(ui, "fractal n", &mut inner.n, 3..=20)
        || add_number_slider(ui, "fractal k", &mut inner.k, 2..=12)
        || add_float_slider(ui, "fractal ra", &mut inner.ra, 0.0..=1.0)
        || add_float_slider_np_length(ui, "fractal ll", &mut inner.ll)
        || add_float_slider_pi(ui, "fractal aa", &mut inner.aa)
}
