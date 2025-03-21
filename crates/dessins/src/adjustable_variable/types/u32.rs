use crate::{
    adjustable_variable::{AdjustableVariable, UpdateVariableParams},
    animation::{Animation, AnimationVariant},
    ui::add_numeric,
};
use bevy::reflect::Reflect;
use nannou::prelude::*;
use std::ops::RangeInclusive;

#[derive(Clone, Debug, PartialEq, Reflect)]
pub struct U32 {
    value: u32,
    range: RangeInclusive<u32>,
    animation: Option<Animation>,
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
                    AnimationVariant::new_sin(
                        self.value as f32,
                        0.1,
                        *self.range.start() as f32,
                        *self.range.end() as f32,
                    ),
                );
                Some(animation)
            }
        }
    }

    pub fn animated(&self) -> bool {
        self.animation.is_some()
    }
}

impl AdjustableVariable for U32 {
    fn update(&mut self, params: UpdateVariableParams) -> bool {
        let UpdateVariableParams {
            ui,
            osc_ctx,
            name,
            time,
        } = params;
        let mut animate = self.animation.is_some();
        let initial_animate = animate;

        // add slider
        let mut changed = add_numeric(ui, &osc_ctx, &name, &mut self.value, self.range.clone());

        // add animate checkbox
        ui.checkbox(&mut animate, "animate");

        if let Some(ref mut animation) = self.animation {
            // animate and...
            self.value = animation.calculate(time).round() as u32;

            // ... add animation params UI elements
            animation.update_ui(ui, self.value as f32, &name);
            changed |= true
        }

        // maybe toggle animate
        if animate != initial_animate {
            self.toggle_animation(time);
        }

        changed
    }
}
