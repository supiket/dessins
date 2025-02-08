use crate::{Segment, Shape, Shapes};
use nannou::prelude::*;
use nannou_egui::egui::Ui;
use ui_controlled_params::UiControlledParams;

#[derive(UiControlledParams)]
#[params(Star)]
pub struct ParamsInner {
    #[param(label = "star k", range(5..=100))]
    pub k: u32, // # vertices
    #[param(label = "star h", range(3..=50))]
    pub h: u32, // # vertices to skip (clockwise) before connecting two dots
    #[param(label = "star r", np, length)]
    pub r: f32, // radius of the circle C on which the vertices are
    #[param(label = "star ad", pi)]
    pub ad: f32, // angle (in radians) of the vector CS with horizontal, where S is the first vertex
}

pub fn calculate_shapes(inner: &ParamsInner) -> Shapes {
    let mut shapes = Shapes::new();
    let mut shape = Shape::new();
    let mut segment = Segment::new();

    for i in 0..inner.k {
        let point = calculate_point(inner, i);
        segment.push(point);
    }

    segment.push(segment[0]);

    shape.push(segment);
    shapes.push(shape);
    shapes
}

pub fn calculate_point(inner: &ParamsInner, i: u32) -> Point2 {
    let angle = (2.0 * i as f32 * inner.h as f32 * PI) / inner.k as f32 + inner.ad;
    let x = inner.r * angle.cos();
    let y = inner.r * angle.sin();
    pt2(x, y)
}
