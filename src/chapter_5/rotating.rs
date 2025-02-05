use crate::{
    add_float_slider, add_float_slider_np, add_number_slider, Model, Segment, Shape, Shapes,
};
use nannou::prelude::*;
use nannou_egui::egui::Ui;
use std::f32::consts::PI;

pub struct CurveParams {
    pub n: u32,  // # segments
    pub t1: f32, // # times the planet turns around the sun
    pub t2: f32, // # times the satellite turns around the planet
    pub r1: f32, // radius of the planet's curve
    pub k1: u32, // elliptic parameter of the planet's curve
    pub k2: u32, // elliptic parameter of the planet's curve
    pub r2: f32, // radius of the satellite's curve
    pub h1: u32, // elliptic parameter of the satellite's curve
    pub h2: u32, // elliptic parameter of the satellite's curve
    pub s_eq: Box<dyn Fn(SParams) -> f32>,
}

pub struct SParams {
    pub i: f32,
    pub n: f32,
}

pub fn update(_app: &App, model: &mut Model<CurveParams>, update: Update) {
    crate::update(model, update, CurveParams::ui_elements)
}

impl CurveParams {
    pub fn ui_elements(&mut self, ui: &mut Ui) -> bool {
        add_number_slider(ui, "curve n", &mut self.n, 1000..=6000)
            || add_float_slider(ui, "curve t1", &mut self.t1, 0.5..=600.0)
            || add_float_slider(ui, "curve t2", &mut self.t2, 0.5..=600.0)
            || add_float_slider_np(ui, "curve r1", &mut self.r1, 0.0..=1.0)
            || add_float_slider_np(ui, "curve r2", &mut self.r2, 0.0..=1.0)
            || add_number_slider(ui, "curve k1", &mut self.k1, 1..=4)
            || add_number_slider(ui, "curve k2", &mut self.k2, 1..=4)
            || add_number_slider(ui, "curve h1", &mut self.h1, 1..=4)
            || add_number_slider(ui, "curve h2", &mut self.h2, 1..=4)
    }

    pub fn calculate_shapes(&self) -> Shapes {
        let mut shapes = Shapes::new();
        let mut shape = Shape::new();
        let mut segment = Segment::new();

        let n = self.n as f32;
        let t1 = self.t1;
        let t2 = self.t2;
        let r1 = self.r1;
        let k1 = self.k1 as f32;
        let k2 = self.k2 as f32;
        let r2 = self.r2;
        let h1 = self.h1 as f32;
        let h2 = self.h2 as f32;

        for i in 0..=self.n {
            let i = i as f32;

            let s = (self.s_eq)(SParams { i, n });
            let an = 2.0 * PI * i / n;
            let c1 = (h1 * an * t1).cos();
            let s1 = (h2 * an * t1).sin();
            let c2 = s * (k1 * an * t2).cos();
            let s2 = s * (k2 * an * t2).sin();

            let x = r1 * c1 + r2 * (c1 * c2 - s1 * s2);
            let y = r1 * s1 + r2 * (s1 * c2 + c1 * s2);

            segment.push(pt2(x, y));
        }

        shape.push(segment);
        shapes.push(shape);

        shapes
    }
}
