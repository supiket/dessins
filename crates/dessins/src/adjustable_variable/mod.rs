use bevy_reflect::Reflect;
use nannou::prelude::*;

pub mod types;

pub trait AdjustableVariable: Reflect {
    fn control(&mut self, ui: &mut egui::Ui, name: &str, time: Time<Virtual>) -> bool;

    fn toggle_animation(&mut self, time: Time<Virtual>);
}
