use crate::{
    adjustable_variable::{AdjustableVariable, UpdateVariableParams},
    animation::Animation,
    shapes::NP,
    ui::{add_float_position, add_numeric},
};
use bevy::reflect::Reflect;
use nannou::prelude::*;

#[derive(Clone, Debug, PartialEq, Reflect)]
pub struct Pt2 {
    value: Point2,
    animation: Pt2Animation,
}

#[derive(Clone, Debug, PartialEq, Reflect)]
pub struct Pt2Animation {
    x: Option<(Animation, AnimationParams)>,
    y: Option<(Animation, AnimationParams)>,
}

#[derive(Clone, Debug, PartialEq, Reflect)]
pub struct AnimationParams {
    freq: f32,
}

impl Pt2 {
    pub fn new(value: Point) -> Self {
        Self {
            value,
            animation: Pt2Animation { x: None, y: None },
        }
    }

    pub fn get_value(&self) -> Point2 {
        self.value
    }

    pub fn set_value(&mut self, value: Point2) {
        self.value = value;
    }
}

impl AdjustableVariable for Pt2 {
    fn update(&mut self, params: UpdateVariableParams) -> bool {
        let UpdateVariableParams { ui, name, time } = params;
        let name_x = format!("{}.x", name);
        let name_y = format!("{}.y", name);

        let x_changed = update(&mut self.value.x, ui, &name_x, time, &mut self.animation.x);
        let y_changed = update(&mut self.value.y, ui, &name_y, time, &mut self.animation.y);

        x_changed | y_changed
    }
}

fn toggle_animation(
    value: f32,
    animation: &mut Option<(Animation, AnimationParams)>,
    time: Time<Virtual>,
) {
    *animation = match animation {
        Some(_) => None,
        None => {
            let animation = Animation::new(time, value, -1.0, 1.0);
            let animation_params = AnimationParams { freq: 0.1 };
            Some((animation, animation_params))
        }
    }
}

fn update(
    value: &mut f32,
    ui: &mut egui::Ui,
    name: &str,
    time: Time<Virtual>,
    animation: &mut Option<(Animation, AnimationParams)>,
) -> bool {
    let mut animate_ = animation.is_some();
    let initial_animate = animate_;

    // add animate checkbox
    ui.checkbox(&mut animate_, "animate");

    // add slider
    ui.label(name);
    let mut changed = add_float_position(ui, value);

    if let Some((animation, ref mut params)) = animation {
        // animate and...
        *value = animate(time, animation, params);

        // ... add animation params UI elements
        add_numeric(ui, "animation frequency", &mut params.freq, 0.0..=1.0);
        changed |= true
    }

    // maybe toggle animate
    if animate_ != initial_animate {
        toggle_animation(*value, animation, time);
    }

    changed
}

fn animate(time: Time<Virtual>, animation: &Animation, params: &AnimationParams) -> f32 {
    let range = -1.0..=1.0;
    let np = NP as f32;
    let range = (*range.start() * np)..=(*range.end() * np);
    animation.update_value_sine(time, params.freq, *range.start(), *range.end())
}
