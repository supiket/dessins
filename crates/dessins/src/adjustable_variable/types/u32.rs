use crate::{
    adjustable_variable::{AdjustableVariable, UpdateVariableParams},
    animation::Animation,
    ui::add_numeric,
};
use bevy::reflect::Reflect;
use nannou::prelude::*;
use std::ops::RangeInclusive;

#[derive(Clone, Debug, PartialEq, Reflect)]
pub struct U32 {
    value: u32,
    range: RangeInclusive<u32>,
    animation: Option<(Animation, AnimationParams)>,
}

#[derive(Clone, Debug, PartialEq, Reflect)]
pub struct AnimationParams {
    freq: f32,
}

impl U32 {
    pub fn new(value: u32, range: RangeInclusive<u32>) -> Self {
        Self {
            value,
            range,
            animation: None,
        }
    }

    pub fn get_value(&self) -> u32 {
        self.value
    }

    pub fn set_value(&mut self, value: u32) {
        self.value = value;
    }

    fn toggle_animation(&mut self, time: Time<Virtual>) {
        self.animation = match self.animation {
            Some(_) => None,
            None => {
                let animation = Animation::new(
                    time,
                    self.value as f32,
                    *self.range.start() as f32,
                    *self.range.end() as f32,
                );
                let animation_params = AnimationParams { freq: 0.1 };
                Some((animation, animation_params))
            }
        }
    }

    pub fn animated(&self) -> bool {
        self.animation.is_some()
    }
}

impl AdjustableVariable for U32 {
    fn update(&mut self, params: UpdateVariableParams) -> bool {
        let UpdateVariableParams { ui, name, time } = params;
        let mut animate = self.animation.is_some();
        let initial_animate = animate;

        // add animate checkbox
        ui.checkbox(&mut animate, "animate");

        // add slider
        let mut changed = add_numeric(ui, &name, &mut self.value, self.range.clone());

        if let Some((animation, ref mut params)) = &mut self.animation {
            // animate and...
            self.value = animation
                .update_value_sine(
                    time,
                    params.freq,
                    *self.range.start() as f32,
                    *self.range.end() as f32,
                )
                .round() as u32;

            // ... add animation params UI elements
            add_numeric(ui, "animation frequency", &mut params.freq, 0.0..=1.0);
            changed |= true
        }

        // maybe toggle animate
        if animate != initial_animate {
            self.toggle_animation(time);
        }

        changed
    }
}
