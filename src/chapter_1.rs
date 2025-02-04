use crate::{add_float_slider, add_float_slider_np, add_float_slider_pi, add_number_slider};
use nannou::prelude::*;
use nannou_egui::egui::Ui;

#[derive(Clone)]
pub struct PolygonSettings {
    pub k: u32,  // # vertices
    pub r: f32,  // radius of the circle on which the vertices are
    pub ad: f32, // angle (in radians) of the vector CS with horizontal, where S is the first vertex
}

pub fn calculate_polygon(settings: &PolygonSettings, i: u32) -> Point2 {
    let angle = (2.0 * i as f32 * PI) / settings.k as f32 + settings.ad;
    let x = settings.r * angle.cos();
    let y = settings.r * angle.sin();
    pt2(x, y)
}

impl PolygonSettings {
    pub fn ui_elements(&mut self, ui: &mut Ui) -> bool {
        add_number_slider(ui, "polygon k", &mut self.k, 3..=20)
            || add_float_slider_np(ui, "polygon r", &mut self.r, 0.0..=1.0)
            || add_float_slider_pi(ui, "polygon ad", &mut self.ad, -1.0..=1.0)
    }
}

#[derive(Clone)]
pub struct StarSettings {
    pub k: u32,  // # vertices
    pub h: u32,  // # vertices to skip (clockwise) before connecting two dots
    pub r: f32,  // radius of the circle C on which the vertices are
    pub ad: f32, // angle (in radians) of the vector CS with horizontal, where S is the first vertex
}

pub fn calculate_stars(settings: &StarSettings, i: u32) -> Point2 {
    let angle = (2.0 * i as f32 * settings.h as f32 * PI) / settings.k as f32 + settings.ad;
    let x = settings.r * angle.cos();
    let y = settings.r * angle.cos();
    pt2(x, y)
}

impl StarSettings {
    pub fn ui_elements(&mut self, ui: &mut Ui) -> bool {
        add_number_slider(ui, "star k", &mut self.k, 5..=100)
            || add_number_slider(ui, "star h", &mut self.h, 3..=50)
            || add_float_slider_np(ui, "star r", &mut self.r, 0.0..=1.0)
            || add_float_slider_pi(ui, "star ad", &mut self.ad, -1.0..=1.0)
    }
}

#[derive(Clone)]
pub struct JolygonSettings {
    pub an: f32, // angle of two consecutive segments
    pub ra: f32, // ratio of the lengths of two consecutive segments
    pub aa: f32, // angle of the first segment with horizontal
}

pub fn calculate_jolygon(
    settings: &JolygonSettings,
    i: u32,
    ref_len: f32,
    ref_pos: Point2,
) -> Point2 {
    let angle = settings.aa + i as f32 * settings.an;

    let dx = ref_len * angle.cos();
    let dy = ref_len * angle.sin();

    let x = ref_pos.x + dx;
    let y = ref_pos.y + dy;

    pt2(x, y)
}

impl JolygonSettings {
    pub fn ui_elements(&mut self, ui: &mut Ui) -> bool {
        add_float_slider_pi(ui, "jolygon an", &mut self.an, -1.0..=1.0)
            || add_float_slider(ui, "jolygon ra", &mut self.ra, 0.0..=1.0)
            || add_float_slider_pi(ui, "jolygon aa", &mut self.aa, 0.0..=1.0)
    }
}
