use crate::NP;
use nannou::prelude::*;
use nannou_egui::{egui, egui::Ui};

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
        let mut recalculate = false;

        ui.label("polygon k:");
        if ui.add(egui::Slider::new(&mut self.k, 3..=20)).changed() {
            recalculate = true;
        }

        ui.label("polygon r:");
        let mut r = self.r / NP as f32;
        if ui
            .add(
                egui::Slider::new(&mut r, 0.0..=1.0)
                    .custom_parser(|str| evalexpr::eval_float(str).ok())
                    .suffix(format!("np (={})", NP)),
            )
            .changed()
        {
            recalculate = true;
        }
        self.r = r * NP as f32;

        ui.label("polygon ad:");
        let mut ad = self.ad / PI;
        if ui
            .add(
                egui::Slider::new(&mut ad, -1.0..=1.00)
                    .custom_parser(|str| evalexpr::eval_float(str).ok())
                    .suffix("π"),
            )
            .changed()
        {
            recalculate = true;
        }
        self.ad = ad * PI;

        recalculate
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
        let mut recalculate = false;

        ui.label("star k:");
        if ui.add(egui::Slider::new(&mut self.k, 5..=100)).changed() {
            recalculate = true;
        }

        ui.label("star h:");
        if ui.add(egui::Slider::new(&mut self.h, 3..=50)).changed() {
            recalculate = true;
        }

        ui.label("star r:");
        let mut r = self.r / NP as f32;
        if ui
            .add(
                egui::Slider::new(&mut r, 0.0..=1.0)
                    .custom_parser(|str| evalexpr::eval_float(str).ok())
                    .suffix(format!("np(={})", NP)),
            )
            .changed()
        {
            recalculate = true;
        }
        self.r = r * NP as f32;

        ui.label("star ad:");
        let mut ad = self.ad / PI;
        if ui
            .add(
                egui::Slider::new(&mut ad, -1.0..=1.00)
                    .custom_parser(|str| evalexpr::eval_float(str).ok())
                    .suffix("π"),
            )
            .changed()
        {
            recalculate = true;
        }
        self.ad = ad * PI;

        recalculate
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
        let mut recalculate = false;

        ui.label("jolygon an:");
        let mut an = self.an / PI;
        if ui
            .add(
                egui::Slider::new(&mut an, -1.0..=1.00)
                    .custom_parser(|str| evalexpr::eval_float(str).ok())
                    .suffix("π"),
            )
            .changed()
        {
            recalculate = true;
        }
        self.an = an * PI;

        ui.label("jolygon ra:");
        if ui
            .add(
                egui::Slider::new(&mut self.ra, 0.0..=1.0)
                    .custom_parser(|str| evalexpr::eval_float(str).ok()),
            )
            .changed()
        {
            recalculate = true;
        }

        ui.label("jolygon aa:");
        if ui
            .add(
                egui::Slider::new(&mut self.aa, 0.0..=PI)
                    .custom_parser(|str| evalexpr::eval_float(str).ok()),
            )
            .changed()
        {
            recalculate = true;
        }

        recalculate
    }
}
