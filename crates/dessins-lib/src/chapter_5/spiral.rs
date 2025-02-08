use crate::{DesignParams, Model, Segment, Shape, Shapes, NP};
use nannou::prelude::*;
use nannou_egui::egui::Ui;
use std::f32::consts::PI;
use ui_controlled_params::UiControlledParams;

#[derive(UiControlledParams)]
pub struct ParamsInner {
    #[param(range(1000..=9000))]
    pub n: u32, // # segments
    #[param(range(40..=60))]
    pub t: u32, // # times the planet turns around the sun
    #[param(length)]
    pub r: f32, // flattening parameter of the ellipse
    #[param(length)]
    pub l: f32, // decrease factor beween the first ellipse traveled and the last
    #[param(range(1.0..=4.0))]
    pub an_factor: f32,
}

pub struct Params {
    pub inner: ParamsInner,
    pub calculate_shapes: Box<dyn Fn(&ParamsInner) -> Shapes>,
    pub ui_elements: UiElements,
}

pub fn model(app: &App, inner: ParamsInner) -> Model {
    let params = DesignParams::Spiral(Params {
        inner,
        calculate_shapes: Box::new(calculate_shapes),
        ui_elements: Box::new(ParamsInner::ui_elements),
    });

    crate::model(params, app)
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
