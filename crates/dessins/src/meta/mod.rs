use crate::animation::AnimationState;
use bevy::reflect::Reflect;
use expression_f32::ExpressionF32;
use f32::F32Type;
use std::{collections::HashMap, ops::RangeInclusive};

pub mod expression_f32;
pub mod f32;

#[derive(Clone, Debug, PartialEq, Reflect)]
pub struct ParamsMeta(pub HashMap<String, ParamMeta>);

#[derive(Clone, Debug, PartialEq, Reflect)]
pub struct ParamMeta {
    pub animation: Option<AnimationState>,
    pub param_type: ParamType,
}

#[derive(Clone, Debug, PartialEq, Reflect)]
pub enum ParamType {
    F32(F32Type),
    ExpressionF32(ExpressionF32),
}

impl ParamMeta {
    pub fn new(param_type: ParamType, animation: Option<AnimationState>) -> Self {
        Self {
            animation,
            param_type,
        }
    }

    pub fn new_f32(f32_type: F32Type) -> Self {
        Self {
            animation: None,
            param_type: ParamType::F32(f32_type),
        }
    }

    pub fn new_f32_from_range(range: RangeInclusive<f32>) -> Self {
        Self {
            animation: None,
            param_type: ParamType::F32(F32Type::new_from_range(range)),
        }
    }
}

impl core::ops::Deref for ParamsMeta {
    type Target = HashMap<String, ParamMeta>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl core::ops::DerefMut for ParamsMeta {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
