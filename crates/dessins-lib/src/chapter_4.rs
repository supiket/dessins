use crate::{Segment, Shape, Shapes};
use nannou::prelude::*;
use nannou_egui::egui::Ui;
use ui_controlled_params::UiControlledParams;

#[derive(UiControlledParams)]
#[params(Fractal)]
pub struct ParamsInner {
    #[param(label="fractal n", range(3..=20))]
    pub n: u32,
    #[param(label="fractal k", range(2..=12))]
    pub k: u32,
    #[param(label = "fractal ra", length)]
    pub ra: f32,
    #[param(label = "fractal ll", np, length)]
    pub ll: f32,
    #[param(label = "fractal aa", pi)]
    pub aa: f32,
    pub p0: Point2,
    pub a0: f32,
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
