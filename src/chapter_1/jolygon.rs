use crate::{add_float_slider, add_float_slider_pi};
use nannou::prelude::*;
use nannou_egui::egui::Ui;

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
