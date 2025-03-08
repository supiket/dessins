use crate::reflect::ControllableParam;
use bevy::reflect::Reflect;
use expression_f32::ExpressionF32;
use f32::F32;
use nannou::prelude::*;
use std::collections::HashMap;

pub mod expression_f32;
pub mod f32;

#[derive(Clone, Debug, PartialEq, Reflect)]
pub struct ParamsMeta(pub HashMap<String, ParamType>);

#[derive(Clone, Debug, PartialEq, Reflect)]
pub enum ParamType {
    F32(F32),
    ExpressionF32(ExpressionF32),
}

impl ControllableParam for ParamType {
    fn control(&mut self, ui: &mut egui::Ui, name: &str) -> bool {
        match self {
            Self::F32(param) => param.control(ui, name),
            Self::ExpressionF32(param) => param.control(ui, name),
        }
    }

    fn toggle_animation_state(&mut self) {
        match self {
            Self::F32(param) => param.toggle_animation_state(),
            Self::ExpressionF32(param) => param.toggle_animation_state(),
        }
    }
}

impl core::ops::Deref for ParamsMeta {
    type Target = HashMap<String, ParamType>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl core::ops::DerefMut for ParamsMeta {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
