use crate::{
    adjustable_variable::{AdjustableVariable, UpdateVariableParams},
    animation::Animation,
    shapes::NP,
    ui::{add_float_length, add_float_pi, add_float_position, add_numeric, float},
};
use bevy::reflect::Reflect;
use nannou::prelude::*;
use std::ops::RangeInclusive;

#[derive(Clone, Debug, PartialEq, Reflect)]
pub struct F32 {
    value: f32,
    variant: F32Variant,
    animation: Option<(Animation, AnimationParams)>,
}

#[derive(Clone, Debug, PartialEq, Reflect)]
pub enum F32Variant {
    None(RangeInclusive<f32>),
    Angle,
    Length,
    Position,
}

#[derive(Clone, Debug, PartialEq, Reflect)]
pub struct AnimationParams {
    freq: f32,
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

    pub fn get_value(&self) -> f32 {
        self.value
    }

    pub fn set_value(&mut self, value: f32) {
        self.value = value;
    }
}

impl AdjustableVariable for F32 {
    fn update(&mut self, params: UpdateVariableParams) -> bool {
        self.variant
            .update(&mut self.value, &mut self.animation, params)
    }
}

impl F32Variant {
    pub fn get_range(&self) -> RangeInclusive<f32> {
        match self {
            Self::None(range) => range.clone(),
            Self::Angle => -2.0..=2.0,
            Self::Length => 0.0..=1.0,
            Self::Position => -1.0..=1.0,
        }
    }
}

impl F32Variant {
    fn update(
        &self,
        value: &mut f32,
        animation: &mut Option<(Animation, AnimationParams)>,
        params: UpdateVariableParams,
    ) -> bool {
        let UpdateVariableParams { ui, name, time } = params;
        let mut animate = animation.is_some();
        let initial_animate = animate;

        // add animate checkbox
        ui.checkbox(&mut animate, "animate");

        // add slider
        let mut changed = self.add_with_label(ui, &name, value);

        if let Some((animation, ref mut params)) = animation {
            // animate and...
            *value = self.animate(time, &animation, &params);

            // ... add animation params UI elements
            add_numeric(ui, "animation frequency", &mut params.freq, 0.0..=1.0);
            changed |= true
        }

        // maybe toggle animate
        if animate != initial_animate {
            self.toggle_animation(*value, animation, time);
        }

        changed
    }
}

impl F32Variant {
    fn add_with_label(&self, ui: &mut egui::Ui, label: &str, value: &mut f32) -> bool {
        match self {
            Self::None(range) => Self::add_variant_none(ui, label, value, range.clone()),
            Self::Position => Self::add_variant_position(ui, label, value),
            Self::Length => Self::add_variant_length(ui, label, value),
            Self::Angle => Self::add_variant_angle(ui, label, value),
        }
    }

    fn animate(
        &self,
        time: Time<Virtual>,
        animation: &Animation,
        animation_params: &AnimationParams,
    ) -> f32 {
        let mut range = self.get_range();
        match self {
            F32Variant::None(_) => {}
            F32Variant::Position | F32Variant::Length => {
                let np = NP as f32;
                range = (*range.start() * np)..=(*range.end() * np);
            }
            F32Variant::Angle => {
                range = (*range.start() * TAU)..=(*range.end() * TAU);
            }
        };
        animation.update_value_sine(time, animation_params.freq, *range.start(), *range.end())
    }

    fn toggle_animation(
        &self,
        value: f32,
        animation: &mut Option<(Animation, AnimationParams)>,
        time: Time<Virtual>,
    ) {
        *animation = match animation {
            Some(_) => None,
            None => {
                let range = self.get_range();
                let animation = Animation::new(time, value, *range.start(), *range.end());
                let animation_params = AnimationParams { freq: 0.1 };
                Some((animation, animation_params))
            }
        }
    }

    fn add_variant_none(
        ui: &mut egui::Ui,
        label: &str,
        value: &mut f32,
        range: RangeInclusive<f32>,
    ) -> bool {
        ui.label(label);
        ui.add(float(value, range)).changed()
    }

    fn add_variant_position(ui: &mut egui::Ui, label: &str, value: &mut f32) -> bool {
        ui.label(label);
        add_float_position(ui, value)
    }

    fn add_variant_length(ui: &mut egui::Ui, label: &str, value: &mut f32) -> bool {
        ui.label(label);
        add_float_length(ui, value)
    }

    fn add_variant_angle(ui: &mut egui::Ui, label: &str, value: &mut f32) -> bool {
        ui.label(label);
        add_float_pi(ui, value)
    }
}
