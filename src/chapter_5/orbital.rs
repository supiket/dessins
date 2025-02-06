use crate::{add_float_slider_np_length, add_number_slider, Model, Segment, Shape, Shapes};
use nannou::prelude::*;
use nannou_egui::egui::Ui;
use std::f32::consts::PI;

pub struct CurveParams {
    pub n: u32,  // # segments
    pub t1: u32, // # times the planet turns around the sun
    pub t2: u32, // # times the satellite turns around the planet
    pub r1: f32, // radius of the planet's curve
    pub k1: u32, // elliptic parameter of the planet's curve
    pub k2: u32, // elliptic parameter of the planet's curve
    pub r2_eq: Box<dyn Fn(R2Params) -> f32>,
}

pub struct R2Params {
    pub i: f32,
    pub n: f32,
}

pub fn update(_app: &App, model: &mut Model<CurveParams>, update: Update) {
    crate::update(model, update, CurveParams::ui_elements)
}

impl CurveParams {
    pub fn ui_elements(&mut self, ui: &mut Ui) -> bool {
        add_number_slider(ui, "curve n", &mut self.n, 1000..=6000)
            || add_number_slider(ui, "curve t1", &mut self.t1, 1..=600)
            || add_number_slider(ui, "curve t2", &mut self.t2, 1..=600)
            || add_float_slider_np_length(ui, "curve r1", &mut self.r1)
            || add_number_slider(ui, "curve k1", &mut self.k1, 1..=4)
            || add_number_slider(ui, "curve k2", &mut self.k2, 1..=4)
    }

    pub fn calculate_shapes(&self) -> Shapes {
        let mut shapes = Shapes::new();
        let mut shape = Shape::new();
        let mut segment = Segment::new();

        let n = self.n as f32;
        let t1 = self.t1 as f32;
        let t2 = self.t2 as f32;
        let r1 = self.r1;
        let k1 = self.k1 as f32;
        let k2 = self.k2 as f32;

        for i in 0..=self.n {
            let i = i as f32;
            let r2 = (self.r2_eq)(R2Params { i, n });
            let a1 = 2.0 * PI * i / n * t1;
            let a2 = 2.0 * PI * i / n * t2;

            let x = r1 * (k1 * a1).cos() + r2 * a2.cos();
            let y = r1 * (k2 * a1).sin() + r2 * a2.sin();

            segment.push(pt2(x, y));
        }

        shape.push(segment);
        shapes.push(shape);

        shapes
    }
}
