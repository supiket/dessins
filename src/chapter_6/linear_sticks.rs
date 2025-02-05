use crate::{add_number_slider, Model, Shape, Shapes};
use nannou::prelude::*;
use nannou_egui::egui::Ui;

pub struct LinearSticksParams {
    pub n: u32,
    pub m: u32,
    pub k: u32,
    pub r1_eq: Box<dyn Fn(&RParams) -> f32>,
    pub r2_eq: Box<dyn Fn(&RParams) -> f32>,
}

pub struct RParams {
    pub i: f32,
}

pub fn update(_app: &App, model: &mut Model<LinearSticksParams>, update: Update) {
    crate::update(model, update, LinearSticksParams::ui_elements)
}

impl LinearSticksParams {
    pub fn calculate_shapes(&self) -> Shapes {
        let mut shapes = Shapes::new();
        let mut shape = Shape::new();

        let n = self.n as f32;
        let k = self.k as f32;

        for i in 0..=self.m {
            let r_params = RParams { i: i as f32 };
            let r1 = (self.r1_eq)(&r_params);
            let r2 = (self.r2_eq)(&r_params);

            for j in 0..self.n {
                let j = j as f32;

                let an = 2.0 * j * PI / n;

                let x = r1 * an.cos() + r2 * (k * an).cos();
                let y = r1 * an.sin() + r2 * (k * an).sin();
                let d = pt2(x, y);

                let x = r1 * an.cos() + r2 * (k * an + PI).cos();
                let y = r1 * an.sin() + r2 * (k * an + PI).sin();
                let a = pt2(x, y);

                let segment = vec![d, a];
                shape.push(segment);
            }
        }

        shapes.push(shape);

        shapes
    }

    pub fn ui_elements(&mut self, ui: &mut Ui) -> bool {
        add_number_slider(ui, "linear modulo n", &mut self.n, 100..=600)
            || add_number_slider(ui, "linear modulo m", &mut self.m, 1..=6)
            || add_number_slider(ui, "linear modulo k", &mut self.k, 3..=7)
    }
}
