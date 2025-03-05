use crate::animation::AnimationState;
use bevy::reflect::Reflect;
use std::{collections::HashMap, ops::RangeInclusive};

#[derive(Clone, Debug, Reflect)]
pub struct ParamsMeta(pub HashMap<String, ParamMeta>);

#[derive(Clone, Debug, Reflect)]
pub struct ParamMeta {
    pub animation: Option<AnimationState>,
    pub subtype: Subtype,
}

#[derive(Clone, Debug, Reflect)]
pub enum Subtype {
    None(RangeStep),
    Angle,
    Length,
    Position,
}

#[derive(Clone, Debug, Reflect)]
pub struct RangeStep {
    pub range: RangeInclusive<f32>,
    pub step: f32,
}

impl ParamMeta {
    pub fn new(range: RangeInclusive<f32>) -> Self {
        Self {
            animation: None,
            subtype: Subtype::None(RangeStep { range, step: 1.0 }),
        }
    }

    pub fn new_angle() -> Self {
        Self {
            animation: None,
            subtype: Subtype::Angle,
        }
    }

    pub fn new_position() -> Self {
        Self {
            animation: None,
            subtype: Subtype::Position,
        }
    }

    pub fn new_length() -> Self {
        Self {
            animation: None,
            subtype: Subtype::Length,
        }
    }
}

impl Subtype {
    pub fn get_range_step(&self) -> RangeStep {
        match self {
            Subtype::None(range_step) => range_step.clone(),
            Subtype::Angle => RangeStep {
                range: -2.0..=2.0,
                step: 0.1,
            },
            Subtype::Length => RangeStep {
                range: 0.0..=1.0,
                step: 0.1,
            },
            Subtype::Position => RangeStep {
                range: -1.0..=1.0,
                step: 0.1,
            },
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
