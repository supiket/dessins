use crate::{
    add_float_slider, add_float_slider_np, add_float_slider_pi, add_number_slider, Model, Segment,
    Shape, Shapes,
};
use nannou::prelude::*;
use nannou_egui::egui::Ui;

pub struct FractalParams {
    pub n: u32,
    pub k: u32,
    pub ra: f32,
    pub ll: f32,
    pub aa: f32,
    pub p0: Point2,
    pub a0: f32,
}

pub fn update(_app: &App, model: &mut Model<FractalParams>, update: Update) {
    crate::update(model, update, FractalParams::ui_elements);
}

impl FractalParams {
    pub fn calculate_shapes(&self) -> Shapes {
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

    pub fn ui_elements(&mut self, ui: &mut Ui) -> bool {
        add_number_slider(ui, "fractal n", &mut self.n, 3..=20)
            || add_number_slider(ui, "fractal k", &mut self.k, 2..=12)
            || add_float_slider(ui, "fractal ra", &mut self.ra, 0.0..=1.0)
            || add_float_slider_np(ui, "fractal ll", &mut self.ll, 0.0..=1.0)
            || add_float_slider_pi(ui, "fractal aa", &mut self.aa, -2.0..=2.0)
    }
}
