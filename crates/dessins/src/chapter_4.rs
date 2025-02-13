use crate::{Segment, Shape, Shapes, NP};
use nannou::prelude::*;
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
    #[param(label = "fractal ll", length)]
    pub ll: f32,
    #[param(label = "fractal aa", pi)]
    pub aa: f32,
    pub p0: Point2,
    pub a0: f32,
}

impl ParamsInner {
    pub fn calculate_shapes(&mut self) -> Shapes {
        let mut shapes = Shapes::new();
        let mut shape = Shape::new();
        let mut segment = Segment::new();

        let mut p0 = self.p0;
        let mut a0 = self.a0;

        let nn = self.n * (self.n - 1).pow(self.k - 1) - 1;

        for i in 0..=nn {
            let mut i1 = i;
            let mut h = 0;

            while i1 % (self.n - 1) == 0 && h < (self.k - 1) {
                i1 /= self.n - 1;
                h += 1;
            }

            let l0 = self.ll * self.ra.powf((self.k - 1 - h) as f32);
            a0 += self.aa;

            let point = p0 + pt2(l0 * a0.cos(), l0 * a0.sin());

            segment.push(point);
            p0 = point;
        }

        shape.push(segment);
        shapes.push(shape);

        shapes
    }
}

impl Default for Params {
    fn default() -> Self {
        let aa = 4.0 * PI / 5.0;
        let ll = NP as f32;

        let a0 = -aa;
        let p0 = pt2((-ll) / 2.0, 0.0);

        Self {
            inner: ParamsInner {
                n: 5,
                k: 5,
                ra: 0.35,
                ll,
                aa,
                a0,
                p0,
            },
            calculate_shapes: Box::new(ParamsInner::calculate_shapes),
            ui_elements: Box::new(ParamsInner::ui_elements),
        }
    }
}
