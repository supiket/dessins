use crate::{add_float_slider, add_float_slider_np, add_number_slider};
use nannou_egui::egui::Ui;

pub struct OrbitalCurveSettings {
    pub n: u32,  // # segments
    pub t1: u32, // # times the planet turns around the sun
    pub t2: u32, // # times the satellite turns around the planet
    pub r1: f32, // radius of the planet's curve
    pub k1: u32, // elliptic parameter of the planet's curve
    pub k2: u32, // elliptic parameter of the planet's curve
}

impl OrbitalCurveSettings {
    pub fn ui_elements(&mut self, ui: &mut Ui) -> bool {
        add_number_slider(ui, "curve n", &mut self.n, 1000..=6000)
            || add_number_slider(ui, "curve t1", &mut self.t1, 1..=600)
            || add_number_slider(ui, "curve t2", &mut self.t2, 1..=600)
            || add_float_slider_np(ui, "curve r1", &mut self.r1, 0.0..=1.0)
            || add_number_slider(ui, "curve k1", &mut self.k1, 1..=4)
            || add_number_slider(ui, "curve k2", &mut self.k2, 1..=4)
    }
}

pub struct RotatingCurveSettings {
    pub n: u32,  // # segments
    pub t1: f32, // # times the planet turns around the sun
    pub t2: f32, // # times the satellite turns around the planet
    pub r1: f32, // radius of the planet's curve
    pub k1: u32, // elliptic parameter of the planet's curve
    pub k2: u32, // elliptic parameter of the planet's curve
    pub r2: f32, // radius of the satellite's curve
    pub h1: u32, // elliptic parameter of the satellite's curve
    pub h2: u32, // elliptic parameter of the satellite's curve
}

impl RotatingCurveSettings {
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
}

pub struct SpiralCurveSettings {
    pub n: u32, // # segments
    pub t: u32, // # times the planet turns around the sun
    pub r: f32, // flattening parameter of the ellipse
    pub l: f32, // decrease factor beween the first ellipse traveled and the last
}

impl SpiralCurveSettings {
    pub fn ui_elements(&mut self, ui: &mut Ui) -> bool {
        add_number_slider(ui, "curve n", &mut self.n, 1000..=9000)
            || add_number_slider(ui, "curve y", &mut self.n, 40..=60)
            || add_float_slider(ui, "curve r", &mut self.r, 0.0..=1.0)
            || add_float_slider(ui, "curve l", &mut self.l, 0.0..=1.0)
    }
}
