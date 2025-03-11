use crate::{
    adjustable_variable::{AdjustableVariable, UpdateVariableParams},
    animation::{Animation, AnimationVariant},
    shapes::NP,
    ui::add_float_position,
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
    x: Option<Animation>,
    y: Option<Animation>,
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

fn toggle_animation(value: f32, animation: &mut Option<Animation>, time: Time<Virtual>) {
    *animation = match animation {
        Some(_) => None,
        None => {
            let animation = Animation::new(
                time,
                AnimationVariant::new_sin(value, 0.1, -1.0 * NP as f32, 1.0 * NP as f32),
            );
            Some(animation)
        }
    }
}

fn update(
    value: &mut f32,
    ui: &mut egui::Ui,
    name: &str,
    time: Time<Virtual>,
    animation: &mut Option<Animation>,
) -> bool {
    let mut animate_ = animation.is_some();
    let initial_animate = animate_;

    // add slider
    ui.label(name);
    let mut changed = add_float_position(ui, value);

    // add animate checkbox
    ui.checkbox(&mut animate_, "animate");

    if let Some(ref mut animation) = animation {
        // animate and...
        *value = animation.calculate(time);

        // ... add animation params UI elements
        animation.update_ui(ui, *value, name);
        changed |= true
    }

    // maybe toggle animate
    if animate_ != initial_animate {
        toggle_animation(*value, animation, time);
    }

    changed
}
