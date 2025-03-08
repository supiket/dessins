use crate::{
    animation::AnimationState,
    reflect::{ControlAction, ControllableParam},
    shapes::NP,
    ui::{add_float_length, add_float_pi, add_float_position, float},
};
use bevy::reflect::Reflect;
use nannou::prelude::*;
use std::ops::RangeInclusive;

#[derive(Clone, Debug, PartialEq, Reflect)]
pub struct F32 {
    pub value: f32,
    pub variant: F32Variant,
    pub animation: Option<AnimationState>,
}

#[derive(Clone, Debug, PartialEq, Reflect)]
pub enum F32Variant {
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

impl F32 {
    pub fn new(value: f32, variant: F32Variant) -> Self {
        let value = match variant {
            F32Variant::Length | F32Variant::Position => value * NP as f32,
            F32Variant::Angle => value * TAU,
            _ => value,
        };
        Self {
            value,
            variant,
            animation: None,
        }
    }

    pub fn new_from_range(value: f32, range: RangeInclusive<f32>) -> Self {
        Self {
            value,
            variant: F32Variant::new_from_range(range),
            animation: None,
        }
    }
}

impl ControllableParam for F32 {
    fn control(&mut self, ui: &mut egui::Ui, name: &str, time: Time<Virtual>) -> bool {
        let ControlAction {
            mut changed,
            toggle_animate,
        } = self
            .variant
            .control_ui(&mut self.value, self.animation.clone(), ui, name);

        if toggle_animate {
            self.toggle_animation_state(time);
        }

        changed |= self
            .variant
            .control_animate(&mut self.value, self.animation.clone(), time);
        changed
    }

    fn toggle_animation_state(&mut self, time: Time<Virtual>) {
        self.animation = match self.animation {
            Some(_) => None,
            None => {
                let RangeStep { range, step } = self.variant.get_range_step();
                let freq = step;
                Some(AnimationState::new(
                    time,
                    freq,
                    *range.start(),
                    *range.end(),
                ))
            }
        }
    }
}

impl F32Variant {
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

impl F32Variant {
    pub fn control_ui(
        &self,
        v: &mut f32,
        animation: Option<AnimationState>,
        ui: &mut egui::Ui,
        name: &str,
    ) -> ControlAction {
        let changed = self.add_with_label(ui, name, v);

        let mut animate = animation.is_some();
        let initial_animate = animate;

        ui.checkbox(&mut animate, "animate");

        let toggle_animate = animate != initial_animate;

        ControlAction {
            changed,
            toggle_animate,
        }
    }

    pub fn control_animate(
        &self,
        v: &mut f32,
        animation: Option<AnimationState>,
        time: Time<Virtual>,
    ) -> bool {
        if let Some(animation_state) = animation {
            *v = self.animate(time, &animation_state);
            true
        } else {
            false
        }
    }
}

impl F32Variant {
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

    pub fn animate(&self, time: Time<Virtual>, animation: &AnimationState) -> f32 {
        let RangeStep {
            mut range,
            mut step,
        } = self.get_range_step();
        match self {
            F32Variant::None(_) => {}
            F32Variant::Position | F32Variant::Length => {
                let np = NP as f32;
                range = (*range.start() * np)..=(*range.end() * np);
                step *= np;
            }
            F32Variant::Angle => {
                range = (*range.start() * TAU)..=(*range.end() * TAU);
                step *= TAU;
            }
        };
        animation.update_value(time, step, *range.start(), *range.end())
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
