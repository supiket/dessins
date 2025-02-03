use crate::NP;
use nannou::geom::{pt2, Point2};
use nannou_egui::{egui, egui::Ui};
use std::f32::consts::PI;

pub struct FractalSettings {
    pub n: usize,
    pub k: usize,
    pub ra: f32,
    pub ll: f32,
    pub aa: f32,
}

impl FractalSettings {
    pub fn calculate_points(&self, p0: Point2, a0: f32) -> Vec<Point2> {
        let mut points = vec![];

        let mut p0 = p0;
        let mut a0 = a0;

        let nn = self.n * (self.n - 1).pow(self.k as u32 - 1) - 1;

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
        let mut recalculate = false;

        ui.label("fractal n:");
        if ui.add(egui::Slider::new(&mut self.n, 3..=20)).changed() {
            recalculate = true;
        }

        ui.label("fractal k:");
        if ui.add(egui::Slider::new(&mut self.k, 2..=12)).changed() {
            recalculate = true;
        }

        ui.label("fractal ra:");
        if ui
            .add(
                egui::Slider::new(&mut self.ra, 0.0..=1.0)
                    .custom_parser(|str| evalexpr::eval_float(str).ok()),
            )
            .changed()
        {
            recalculate = true;
        }

        ui.label("fractal ll:");
        let mut ll = self.ll / NP as f32;
        if ui
            .add(
                egui::Slider::new(&mut ll, 0.0..=1.0)
                    .custom_parser(|str| evalexpr::eval_float(str).ok())
                    .suffix(format!("np (={})", NP)),
            )
            .changed()
        {
            recalculate = true;
        }
        self.ll = ll * NP as f32;

        ui.label("fractal aa:");
        let mut aa = self.aa / PI;
        if ui
            .add(
                egui::Slider::new(&mut aa, -2.0..=2.0)
                    .custom_parser(|str| evalexpr::eval_float(str).ok())
                    .suffix("π"),
            )
            .changed()
        {
            recalculate = true;
        }
        self.aa = aa * PI;

        recalculate
    }
}
