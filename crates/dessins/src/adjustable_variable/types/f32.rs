use crate::{
    adjustable_variable::{AdjustableVariable, UpdateVariableParams},
    animation::{Animation, AnimationVariant},
    shapes::NP,
    ui::{add_float_length, add_float_pi, add_float_position, float},
};
use bevy::reflect::Reflect;
use nannou::prelude::*;
use std::ops::RangeInclusive;

#[derive(Clone, Debug, PartialEq, Reflect)]
pub struct F32 {
    value: f32,
    variant: F32Variant,
    animation: Option<Animation>,
}

#[derive(Clone, Debug, PartialEq, Reflect)]
pub enum F32Variant {
    None(RangeInclusive<f32>),
    Angle,
    Length,
    Position,
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
    pub fn get_value_range(&self) -> RangeInclusive<f32> {
        let np = NP as f32;
        match self {
            Self::None(range) => range.clone(),
            Self::Angle => -TAU..=TAU,
            Self::Length => 0.0 * np..=1.0 * np,
            Self::Position => -1.0 * np..=1.0 * np,
        }
    }
}

impl F32Variant {
    fn update(
        &self,
        value: &mut f32,
        animation: &mut Option<Animation>,
        params: UpdateVariableParams,
    ) -> bool {
        let UpdateVariableParams { ui, name, time } = params;
        let mut animate = animation.is_some();
        let initial_animate = animate;

        // add slider
        let mut changed = self.add_with_label(ui, &name, value);

        // add animate checkbox
        ui.checkbox(&mut animate, "animate");

        if let Some(ref mut animation) = animation {
            // animate and...
            *value = animation.calculate(time);

            // ... add animation params UI elements
            animation.update_ui(ui, *value, &name);
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

    fn toggle_animation(&self, value: f32, animation: &mut Option<Animation>, time: Time<Virtual>) {
        *animation = match animation {
            Some(_) => None,
            None => {
                let range = self.get_value_range();
                let animation = Animation::new(
                    time,
                    AnimationVariant::new_sin(value, 0.1, *range.start(), *range.end()),
                );
                Some(animation)
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
