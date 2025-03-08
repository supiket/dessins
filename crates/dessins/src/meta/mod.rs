use crate::reflect::ControllableParam;
use bevy::reflect::Reflect;
use expression_f32::ExpressionF32;
use f32::F32;
use nannou::prelude::*;

pub mod expression_f32;
pub mod f32;

#[derive(Clone, Debug, PartialEq, Reflect)]
pub enum VariableType {
    F32(F32),
    ExpressionF32(ExpressionF32),
}

impl ControllableParam for VariableType {
    fn control(&mut self, ui: &mut egui::Ui, name: &str, time: Time<Virtual>) -> bool {
        match self {
            Self::F32(param) => param.control(ui, name, time),
            Self::ExpressionF32(param) => param.control(ui, name, time),
        }
    }

    fn toggle_animation(&mut self, time: Time<Virtual>) {
        match self {
            Self::F32(param) => param.toggle_animation(time),
            Self::ExpressionF32(param) => param.toggle_animation(time),
        }
    }
}
