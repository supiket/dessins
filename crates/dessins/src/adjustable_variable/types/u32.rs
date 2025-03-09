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
    step: u32,
    animation: Option<Animation>,
}

impl U32 {
    pub fn new(value: u32, range: RangeInclusive<u32>, step: u32) -> Self {
        Self {
            value,
            range,
            step,
            animation: None,
        }
    }

    pub fn get_value(&self) -> u32 {
        self.value
    }

    pub fn set_value(&mut self, value: u32) {
        self.value = value;
    }

    fn update_ui(&mut self, params: UpdateVariableParams) -> bool {
        let UpdateVariableParams { ui, name, time } = params;

        let recalculate_points = add_numeric(ui, &name, &mut self.value, self.range.clone());

        let mut animate = self.animation.is_some();
        let initial_animate = animate;

        ui.checkbox(&mut animate, "animate");

        let toggle_animate = animate != initial_animate;

        if toggle_animate {
            self.toggle_animation(time);
        }

        recalculate_points
    }

    fn update_animate(&mut self, time: Time<Virtual>) -> bool {
        if let Some(animation) = &self.animation {
            self.value = animation
                .update_value(
                    time,
                    self.step as f32,
                    *self.range.start() as f32,
                    *self.range.end() as f32,
                )
                .round() as u32;
            true
        } else {
            false
        }
    }

    fn toggle_animation(&mut self, time: Time<Virtual>) {
        self.animation = match self.animation {
            Some(_) => None,
            None => {
                let freq = self.step as f32;
                Some(Animation::new(
                    time,
                    freq,
                    *self.range.start() as f32,
                    *self.range.end() as f32,
                ))
            }
        }
    }

    pub fn animated(&self) -> bool {
        self.animation.is_some()
    }
}

impl AdjustableVariable for U32 {
    fn update(&mut self, params: UpdateVariableParams) -> bool {
        let time = params.time;

        let mut recalculate_points = self.update_ui(params);

        recalculate_points |= self.update_animate(time);
        recalculate_points
    }
}
