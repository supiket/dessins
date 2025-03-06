use crate::{
    animation::AnimationState,
    shapes::NP,
    ui::{add_float_length, add_float_pi, add_float_position, float},
};
use bevy::reflect::Reflect;
use nannou::prelude::*;
use std::ops::RangeInclusive;

#[derive(Clone, Debug, PartialEq, Reflect)]
pub enum F32Type {
    None(RangeStep),
    Angle,
    Length,
    Position,
}

#[derive(Clone, Debug, PartialEq, Reflect)]
pub struct RangeStep {
    pub range: RangeInclusive<f32>,
    pub step: f32,
}

impl F32Type {
    pub fn new_from_range(range: RangeInclusive<f32>) -> Self {
        Self::None(RangeStep { range, step: 1.0 })
    }

    pub fn get_range_step(&self) -> RangeStep {
        match self {
            Self::None(range_step) => range_step.clone(),
            Self::Angle => RangeStep {
                range: -2.0..=2.0,
                step: 0.1,
            },
            Self::Length => RangeStep {
                range: 0.0..=1.0,
                step: 0.1,
            },
            Self::Position => RangeStep {
                range: -1.0..=1.0,
                step: 0.1,
            },
        }
    }
}

impl F32Type {
    pub fn add_with_label(&self, ui: &mut egui::Ui, label: &str, value: &mut f32) -> bool {
        match self {
            Self::None(RangeStep { range, step: _step }) => {
                Self::add_variant_none(ui, label, value, range.clone())
            }
            Self::Position => Self::add_variant_position(ui, label, value),
            Self::Length => Self::add_variant_length(ui, label, value),
            Self::Angle => Self::add_variant_angle(ui, label, value),
        }
    }

    pub fn animate(&self, animation: &AnimationState) -> f32 {
        // TODO: transform between coefficients and actual values
        let RangeStep { range, step } = match self {
            F32Type::None(range_step) => range_step,
            F32Type::Position => &RangeStep {
                range: -(NP as f32)..=NP as f32,
                step: 1.0,
            },
            F32Type::Length => &RangeStep {
                range: 0.0..=NP as f32,
                step: 1.0,
            },
            F32Type::Angle => &RangeStep {
                range: -TAU..=TAU,
                step: 1.0,
            },
        };
        animation.update_value(*step, *range.start(), *range.end())
    }

    pub fn add_variant_none(
        ui: &mut egui::Ui,
        label: &str,
        value: &mut f32,
        range: RangeInclusive<f32>,
    ) -> bool {
        ui.label(label);
        ui.add(float(value, range)).changed()
    }

    pub fn add_variant_position(ui: &mut egui::Ui, label: &str, value: &mut f32) -> bool {
        ui.label(label);
        add_float_position(ui, value)
    }

    pub fn add_variant_length(ui: &mut egui::Ui, label: &str, value: &mut f32) -> bool {
        ui.label(label);
        add_float_length(ui, value)
    }

    pub fn add_variant_angle(ui: &mut egui::Ui, label: &str, value: &mut f32) -> bool {
        ui.label(label);
        add_float_pi(ui, value)
    }
}
