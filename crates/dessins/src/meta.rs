use crate::animation::AnimationState;
use std::{collections::HashMap, ops::RangeInclusive};

pub type FieldsMeta = HashMap<String, FieldMeta>;

#[derive(Clone, Debug)]
pub struct FieldMeta {
    pub animation: Option<AnimationState>,
    pub subtype: Subtype,
}

#[derive(Clone, Debug)]
pub enum Subtype {
    None(RangeStep),
    Angle,
    Length,
    Position,
}

#[derive(Clone, Debug)]
pub struct RangeStep {
    pub range: RangeInclusive<f32>,
    pub step: f32,
}

pub trait ParamMeta {
    fn get_fields_meta(&self) -> Option<FieldsMeta>;
    fn set_fields_meta(&mut self, path: &str);
    fn toggle_field_animation_state(&mut self, field_key: &str) -> anyhow::Result<()>;
}

impl FieldMeta {
    pub fn new(range: RangeInclusive<f32>, step: f32) -> Self {
        Self {
            animation: None,
            subtype: Subtype::None(RangeStep { range, step }),
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
    pub fn get_range(&self) -> RangeInclusive<f32> {
        match self {
            Subtype::None(RangeStep { range, step: _ }) => range.clone(),
            Subtype::Angle => -2.0..=2.0,
            Subtype::Length => 0.0..=1.0,
            Subtype::Position => -1.0..=1.0,
        }
    }
}
