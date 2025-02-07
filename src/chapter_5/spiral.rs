use crate::{add_float_slider, add_number_slider, DesignParams, Model, Segment, Shape, Shapes, NP};
use nannou::prelude::*;
use nannou_egui::egui::Ui;
use std::f32::consts::PI;

pub struct ParamsInner {
    pub n: u32, // # segments
    pub t: u32, // # times the planet turns around the sun
    pub r: f32, // flattening parameter of the ellipse
    pub l: f32, // decrease factor beween the first ellipse traveled and the last
    pub an_factor: f32,
}

pub struct Params {
    pub inner: ParamsInner,
    pub calculate_shapes: Box<dyn Fn(&ParamsInner) -> Shapes>,
    pub ui_elements: Box<dyn Fn(&mut ParamsInner, &mut Ui) -> bool>,
}

pub fn model(app: &App, inner: ParamsInner) -> Model {
    let params = DesignParams::Spiral(Params {
        inner,
        calculate_shapes: Box::new(calculate_shapes),
        ui_elements: Box::new(ui_elements),
    });

    crate::model(params, app)
}

pub fn ui_elements(inner: &mut ParamsInner, ui: &mut Ui) -> bool {
    add_number_slider(ui, "curve n", &mut inner.n, 1000..=9000)
        || add_number_slider(ui, "curve t", &mut inner.t, 40..=60)
        || add_float_slider(ui, "curve r", &mut inner.r, 0.0..=1.0)
        || add_float_slider(ui, "curve l", &mut inner.l, 0.0..=1.0)
        || add_float_slider(ui, "curve an factor", &mut inner.an_factor, 1.0..=4.0)
}

pub fn calculate_shapes(inner: &ParamsInner) -> Shapes {
    let mut shapes = Shapes::new();
    let mut shape = Shape::new();
    let mut segment = Segment::new();

    let np = NP as f32;
    let n = inner.n as f32;
    let t = inner.t as f32;
    let r = inner.r;
    let l = inner.l;
    let an_factor = inner.an_factor;

    for i in 0..=inner.n {
        let i = i as f32;

        let rr = l.powf(i / n);
        let an = 2.0 * PI * i / n * an_factor;

        let x = rr * (t * an).cos();
        let y = rr * r * (t * an).sin();

        let co = an.cos();
        let si = an.sin();

        let xx = x * co - y * si;
        let yy = x * si + y * co;

        let x = xx * np / 2.0;
        let y = yy * np / 2.0;

        segment.push(pt2(x, y));
    }

    shape.push(segment);
    shapes.push(shape);

    shapes
}
