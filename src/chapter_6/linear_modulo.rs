use crate::{add_float_slider, add_number_slider, Model, Segment, Shape, Shapes, NP};
use nannou::prelude::*;
use nannou_egui::egui::Ui;

pub struct LinearModuloParams {
    pub n: u32,
    pub m: u32,
    pub k1: f32,
    pub k2: f32,
    pub h: u32,
    pub i1_factor: u32,
    pub y_eq: Box<dyn Fn(&YParams) -> f32>,
}

pub struct YParams {
    pub i: f32,
    pub n: f32,
    pub k2: f32,
}

pub fn update(_app: &App, model: &mut Model<LinearModuloParams>, update: Update) {
    crate::update(model, update, LinearModuloParams::ui_elements)
}

impl LinearModuloParams {
    fn calculate_points(&self) -> Segment {
        let mut points = vec![];

        let n = self.n as f32;
        let k1 = self.k1 as f32;
        let k2 = self.k2 as f32;

        for i in 0..=self.n {
            let i = i as f32;

            let x = NP as f32 * 0.5 * (k1 * i * PI / n).sin();
            let y = (self.y_eq)(&YParams { i, n, k2 });
            points.push(pt2(x, y));
        }

        points
    }

    pub fn calculate_shapes(&self) -> Shapes {
        let mut shapes = Shapes::new();
        let mut shape = Shape::new();

        let points = self.calculate_points();

        for i in 0..=self.m {
            let start_index = ((self.i1_factor * i) % self.n) as usize;
            let end_index = ((self.h * i) % self.n) as usize;

            let segment = vec![points[start_index], points[end_index]];
            shape.push(segment);
        }

        shapes.push(shape);

        shapes
    }

    pub fn ui_elements(&mut self, ui: &mut Ui) -> bool {
        add_number_slider(ui, "linear modulo n", &mut self.n, 10..=400)
            || add_number_slider(ui, "linear modulo m", &mut self.m, 10..=400)
            || add_float_slider(ui, "linear modulo k1", &mut self.k1, 1.0..=5.0)
            || add_float_slider(ui, "linear modulo k2", &mut self.k2, 1.0..=5.0)
            || add_number_slider(ui, "linear modulo h", &mut self.h, 1..=10)
            || add_number_slider(ui, "linear modulo i1 factor", &mut self.i1_factor, 1..=8)
    }
}
