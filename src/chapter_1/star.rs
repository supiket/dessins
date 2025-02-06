use crate::{add_float_slider_np_length, add_float_slider_pi, add_number_slider};
use nannou::prelude::*;
use nannou_egui::egui::Ui;

#[derive(Clone)]
pub struct StarParams {
    pub k: u32,  // # vertices
    pub h: u32,  // # vertices to skip (clockwise) before connecting two dots
    pub r: f32,  // radius of the circle C on which the vertices are
    pub ad: f32, // angle (in radians) of the vector CS with horizontal, where S is the first vertex
}

pub fn calculate_stars(params: &StarParams, i: u32) -> Point2 {
    let angle = (2.0 * i as f32 * params.h as f32 * PI) / params.k as f32 + params.ad;
    let x = params.r * angle.cos();
    let y = params.r * angle.sin();
    pt2(x, y)
}

impl StarParams {
    pub fn ui_elements(&mut self, ui: &mut Ui) -> bool {
        add_number_slider(ui, "star k", &mut self.k, 5..=100)
            || add_number_slider(ui, "star h", &mut self.h, 3..=50)
            || add_float_slider_np_length(ui, "star r", &mut self.r)
            || add_float_slider_pi(ui, "star ad", &mut self.ad)
    }
}
