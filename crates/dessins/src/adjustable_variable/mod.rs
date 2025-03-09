use bevy_reflect::Reflect;
use nannou::prelude::*;

pub mod types;

pub struct UpdateVariableParams<'a> {
    pub ui: &'a mut egui::Ui,
    pub name: String,
    pub time: Time<Virtual>,
}

pub trait AdjustableVariable: Reflect {
    fn update(&mut self, params: UpdateVariableParams) -> bool;

    fn toggle_animation(&mut self, time: Time<Virtual>);
}
