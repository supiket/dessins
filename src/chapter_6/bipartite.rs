use crate::{
    add_float_slider_np_position, add_number_slider, DesignParams, Model, Segment, Shape, Shapes,
};
use nannou::prelude::*;
use nannou_egui::egui::Ui;

pub type OuterSegment = Segment;
pub type InnerSegment = Segment;

pub struct ParamsInner {
    pub n: u32,
    pub a: Point2,
    pub b: Point2,
    pub c: Point2,
    pub d: Point2,
}

pub struct YParams {
    pub i: f32,
    pub n: f32,
    pub k2: f32,
}

pub struct Params {
    pub inner: ParamsInner,
    pub calculate_shapes: Box<dyn Fn(&ParamsInner) -> Shapes>,
    pub ui_elements: Box<dyn Fn(&mut ParamsInner, &mut Ui) -> bool>,
}

pub fn model(app: &App, inner: ParamsInner) -> Model {
    let params = DesignParams::Bipartite(Params {
        inner,
        calculate_shapes: Box::new(calculate_shapes),
        ui_elements: Box::new(ui_elements),
    });

    crate::model(params, app)
}

pub fn ui_elements(inner: &mut ParamsInner, ui: &mut Ui) -> bool {
    add_number_slider(ui, "bipartite n", &mut inner.n, 10..=20)
        || add_float_slider_np_position(ui, "bipartite a.x", &mut inner.a.x)
        || add_float_slider_np_position(ui, "bipartite a.y", &mut inner.a.y)
        || add_float_slider_np_position(ui, "bipartite b.x", &mut inner.b.x)
        || add_float_slider_np_position(ui, "bipartite b.y", &mut inner.b.y)
        || add_float_slider_np_position(ui, "bipartite c.x", &mut inner.c.x)
        || add_float_slider_np_position(ui, "bipartite c.y", &mut inner.c.y)
        || add_float_slider_np_position(ui, "bipartite d.x", &mut inner.d.x)
        || add_float_slider_np_position(ui, "bipartite d.y", &mut inner.d.y)
}

pub fn calculate_shapes(inner: &ParamsInner) -> Shapes {
    let mut shapes = Shapes::new();
    let mut shape = Shape::new();

    let (outer_points, inner_points) = calculate_points(inner);

    for outer in &outer_points {
        for inner in &inner_points {
            let segment = vec![*outer, *inner];
            shape.push(segment);
        }
    }

    shapes.push(shape);

    shapes
}

fn calculate_points(inner: &ParamsInner) -> (OuterSegment, InnerSegment) {
    let mut outer_segment = vec![];
    let mut inner_segment = vec![];

    let n = inner.n as f32;

    for i in 0..=inner.n {
        let i = i as f32;
        let x1 = (i * inner.a.x + (n - i) * inner.b.x) / n;
        let y1 = (i * inner.a.y + (n - i) * inner.b.y) / n;
        outer_segment.push(pt2(x1, y1));

        for j in 0..=inner.n {
            let j = j as f32;

            let x2 = (j * inner.c.x + (n - j) * inner.d.x) / n;
            let y2 = (j * inner.c.y + (n - j) * inner.d.y) / n;
            inner_segment.push(pt2(x2, y2));
        }
    }

    (outer_segment, inner_segment)
}
