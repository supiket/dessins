use crate::NP;
use nannou_egui::{egui, egui::Ui};

pub struct OrbitalCurveSettings {
    pub n: usize,  // # segments
    pub t1: usize, // # times the planet turns around the sun
    pub t2: usize, // # times the satellite turns around the planet
    pub r1: f32,   // radius of the planet's curve
    pub k1: u32,   // elliptic parameter of the planet's curve
    pub k2: u32,   // elliptic parameter of the planet's curve
}

impl OrbitalCurveSettings {
    pub fn ui_elements(&mut self, ui: &mut Ui) -> bool {
        let mut recalculate = false;

        ui.label("curve n:");
        if ui
            .add(egui::Slider::new(&mut self.n, 1000..=6000))
            .changed()
        {
            recalculate = true;
        }

        ui.label("curve t1:");
        if ui.add(egui::Slider::new(&mut self.t1, 1..=600)).changed() {
            recalculate = true;
        }

        ui.label("curve t2:");
        if ui.add(egui::Slider::new(&mut self.t2, 1..=600)).changed() {
            recalculate = true;
        }

        ui.label("curve r1:");
        let mut r1 = self.r1 / NP as f32;
        if ui
            .add(
                egui::Slider::new(&mut r1, 0.0..=1.0)
                    .custom_parser(|str| evalexpr::eval_float(str).ok())
                    .suffix(format!("np (={})", NP)),
            )
            .changed()
        {
            recalculate = true;
        }
        self.r1 = r1 * NP as f32;

        ui.label("curve k1:");
        if ui
            .add(
                egui::Slider::new(&mut self.k1, 1..=4)
                    .custom_parser(|str| evalexpr::eval_float(str).ok()),
            )
            .changed()
        {
            recalculate = true;
        }

        ui.label("curve k2:");
        if ui
            .add(
                egui::Slider::new(&mut self.k2, 1..=4)
                    .custom_parser(|str| evalexpr::eval_float(str).ok()),
            )
            .changed()
        {
            recalculate = true;
        }

        recalculate
    }
}
