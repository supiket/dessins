use crate::{add_float_slider, add_float_slider_np, add_float_slider_pi, add_number_slider};
use nannou::geom::{pt2, Point2};
use nannou_egui::egui::Ui;

pub struct FractalSettings {
    pub n: u32,
    pub k: u32,
    pub ra: f32,
    pub ll: f32,
    pub aa: f32,
}

impl FractalSettings {
    pub fn calculate_points(&self, p0: Point2, a0: f32) -> Vec<Point2> {
        let mut points = vec![];

        let mut p0 = p0;
        let mut a0 = a0;

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

            points.push(point);
            p0 = point;
        }

        points
    }

    pub fn ui_elements(&mut self, ui: &mut Ui) -> bool {
        add_number_slider(ui, "fractal n", &mut self.n, 3..=20)
            || add_number_slider(ui, "fractal k", &mut self.k, 2..=12)
            || add_float_slider(ui, "fractal ra", &mut self.ra, 0.0..=1.0)
            || add_float_slider_np(ui, "fractal ll", &mut self.ll, 0.0..=1.0)
            || add_float_slider_pi(ui, "fractal aa", &mut self.aa, -2.0..=2.0)
    }
}
