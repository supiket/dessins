use crate::{add_float_slider_np_length, add_float_slider_pi, add_number_slider};
use nannou::prelude::*;
use nannou_egui::egui::Ui;

#[derive(Clone)]
pub struct PolygonParams {
    pub k: u32,  // # vertices
    pub r: f32,  // radius of the circle on which the vertices are
    pub ad: f32, // angle (in radians) of the vector CS with horizontal, where S is the first vertex
}

pub fn calculate_polygon(params: &PolygonParams, i: u32) -> Point2 {
    let angle = (2.0 * i as f32 * PI) / params.k as f32 + params.ad;
    let x = params.r * angle.cos();
    let y = params.r * angle.sin();
    pt2(x, y)
}

impl PolygonParams {
    pub fn ui_elements(&mut self, ui: &mut Ui) -> bool {
        add_number_slider(ui, "polygon k", &mut self.k, 3..=20)
            || add_float_slider_np_length(ui, "polygon r", &mut self.r)
            || add_float_slider_pi(ui, "polygon ad", &mut self.ad)
    }
}
