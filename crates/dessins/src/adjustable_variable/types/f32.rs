use super::AdjustVariable;
use crate::{
    adjustable_variable::{AdjustableVariable, UpdateVariableParams},
    animation::Animation,
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

    pub fn get_value(&self) -> f32 {
        self.value
    }

    pub fn set_value(&mut self, value: f32) {
        self.value = value;
    }

    pub fn new_from_range(value: f32, range: RangeInclusive<f32>) -> Self {
        Self {
            value,
            variant: F32Variant::new_from_range(range),
            animation: None,
        }
    }

    fn toggle_animation(&mut self, time: Time<Virtual>) {
        self.animation = match self.animation {
            Some(_) => None,
            None => {
                let RangeStep { range, step } = self.variant.get_range_step();
                let freq = step;
                Some(Animation::new(time, freq, *range.start(), *range.end()))
            }
        }
    }
}

impl AdjustableVariable for F32 {
    fn update(&mut self, params: UpdateVariableParams) -> bool {
        let UpdateVariableParams { ui, time, name } = params;

        let AdjustVariable {
            mut recalculate_points,
            toggle_animate,
        } = self
            .variant
            .update_ui(&mut self.value, self.animation.clone(), ui, &name);

        if toggle_animate {
            self.toggle_animation(time);
        }

        recalculate_points |=
            self.variant
                .update_animate(&mut self.value, self.animation.clone(), time);
        recalculate_points
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
    fn update_ui(
        &self,
        v: &mut f32,
        animation: Option<Animation>,
        ui: &mut egui::Ui,
        name: &str,
    ) -> AdjustVariable {
        let recalculate_points = self.add_with_label(ui, name, v);

        let mut animate = animation.is_some();
        let initial_animate = animate;

        ui.checkbox(&mut animate, "animate");

        let toggle_animate = animate != initial_animate;

        AdjustVariable {
            recalculate_points,
            toggle_animate,
        }
    }

    fn update_animate(
        &self,
        v: &mut f32,
        animation: Option<Animation>,
        time: Time<Virtual>,
    ) -> bool {
        if let Some(animation) = animation {
            *v = self.animate(time, &animation);
            true
        } else {
            false
        }
    }
}

impl F32Variant {
    fn add_with_label(&self, ui: &mut egui::Ui, label: &str, value: &mut f32) -> bool {
        match self {
            Self::None(RangeStep { range, step: _step }) => {
                Self::add_variant_none(ui, label, value, range.clone())
            }
            Self::Position => Self::add_variant_position(ui, label, value),
            Self::Length => Self::add_variant_length(ui, label, value),
            Self::Angle => Self::add_variant_angle(ui, label, value),
        }
    }

    fn animate(&self, time: Time<Virtual>, animation: &Animation) -> f32 {
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
