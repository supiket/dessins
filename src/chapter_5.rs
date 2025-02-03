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

pub struct RotatingCurveSettings {
    pub n: usize, // # segments
    pub t1: f32,  // # times the planet turns around the sun
    pub t2: f32,  // # times the satellite turns around the planet
    pub r1: f32,  // radius of the planet's curve
    pub k1: u32,  // elliptic parameter of the planet's curve
    pub k2: u32,  // elliptic parameter of the planet's curve
    pub r2: f32,  // radius of the satellite's curve
    pub h1: u32,  // elliptic parameter of the satellite's curve
    pub h2: u32,  // elliptic parameter of the satellite's curve
}

impl RotatingCurveSettings {
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
        if ui
            .add(
                egui::Slider::new(&mut self.t1, 0.5..=600.0)
                    .custom_parser(|str| evalexpr::eval_float(str).ok()),
            )
            .changed()
        {
            recalculate = true;
        }

        ui.label("curve t2:");
        if ui
            .add(
                egui::Slider::new(&mut self.t2, 0.5..=600.0)
                    .custom_parser(|str| evalexpr::eval_float(str).ok()),
            )
            .changed()
        {
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

        ui.label("curve r2:");
        let mut r2 = self.r2 / NP as f32;
        if ui
            .add(
                egui::Slider::new(&mut r2, 0.0..=1.0)
                    .custom_parser(|str| evalexpr::eval_float(str).ok())
                    .suffix(format!("np (={})", NP)),
            )
            .changed()
        {
            recalculate = true;
        }
        self.r2 = r2 * NP as f32;

        ui.label("curve h1:");
        if ui
            .add(
                egui::Slider::new(&mut self.h1, 1..=4)
                    .custom_parser(|str| evalexpr::eval_float(str).ok()),
            )
            .changed()
        {
            recalculate = true;
        }

        ui.label("curve h2:");
        if ui
            .add(
                egui::Slider::new(&mut self.h2, 1..=4)
                    .custom_parser(|str| evalexpr::eval_float(str).ok()),
            )
            .changed()
        {
            recalculate = true;
        }

        recalculate
    }
}

pub struct SpiralCurveSettings {
    pub n: usize, // # segments
    pub t: u32,   // # times the planet turns around the sun
    pub r: f32,   // flattening parameter of the ellipse
    pub l: f32,   // decrease factor beween the first ellipse traveled and the last
}

impl SpiralCurveSettings {
    pub fn ui_elements(&mut self, ui: &mut Ui) -> bool {
        let mut recalculate = false;

        ui.label("curve n:");
        if ui
            .add(egui::Slider::new(&mut self.n, 1000..=9000))
            .changed()
        {
            recalculate = true;
        }

        ui.label("curve t:");
        if ui
            .add(
                egui::Slider::new(&mut self.t, 40..=60)
                    .custom_parser(|str| evalexpr::eval_float(str).ok()),
            )
            .changed()
        {
            recalculate = true;
        }

        ui.label("curve r:");
        if ui
            .add(
                egui::Slider::new(&mut self.r, 0.0..=1.0)
                    .custom_parser(|str| evalexpr::eval_float(str).ok()),
            )
            .changed()
        {
            recalculate = true;
        }

        ui.label("curve l:");
        if ui
            .add(
                egui::Slider::new(&mut self.l, 0.0..=1.0)
                    .custom_parser(|str| evalexpr::eval_float(str).ok()),
            )
            .changed()
        {
            recalculate = true;
        }

        recalculate
    }
}
