use crate::{
    adjustable_variable::{AdjustableVariable, UpdateVariableParams},
    animation::Animation,
    shapes::NP,
    ui::add_float_position,
};
use bevy::reflect::Reflect;
use nannou::prelude::*;

#[derive(Clone, Debug, PartialEq, Reflect)]
pub struct Pt2 {
    value: Point2,
    animation: (Option<Animation>, Option<Animation>),
}

impl Pt2 {
    pub fn new(value: Point) -> Self {
        Self {
            value,
            animation: (None, None),
        }
    }

    pub fn get_value(&self) -> Point2 {
        self.value
    }

    pub fn set_value(&mut self, value: Point2) {
        self.value = value;
    }

    fn update_ui(&mut self, params: UpdateVariableParams) -> bool {
        let UpdateVariableParams { ui, name, time } = params;
        let name_x = format!("{}.x", name);
        let name_y = format!("{}.y", name);
        let recalculate_x = update_ui(&mut self.value.x, &mut self.animation.0, time, ui, &name_x);
        let recalculate_y = update_ui(&mut self.value.y, &mut self.animation.1, time, ui, &name_y);
        recalculate_x | recalculate_y
    }

    fn update_animate(&mut self, time: Time<Virtual>) -> bool {
        let recalculate_x = update_animate(&mut self.value.x, &self.animation.0, time);
        let recalculate_y = update_animate(&mut self.value.y, &self.animation.1, time);
        recalculate_x | recalculate_y
    }
}

impl AdjustableVariable for Pt2 {
    fn update(&mut self, params: UpdateVariableParams) -> bool {
        let UpdateVariableParams { time, .. } = params;

        let mut recalculate_points = self.update_ui(params);

        recalculate_points |= self.update_animate(time);
        recalculate_points
    }
}

fn toggle_animation(animation: &mut Option<Animation>, time: Time<Virtual>) {
    *animation = match animation {
        Some(_) => None,
        None => {
            let range = -1.0..=1.0;
            let step = 0.1;
            let freq = step;
            Some(Animation::new(time, freq, *range.start(), *range.end()))
        }
    }
}

fn update_ui(
    value: &mut f32,
    animation: &mut Option<Animation>,
    time: Time<Virtual>,
    ui: &mut egui::Ui,
    name: &str,
) -> bool {
    ui.label(name);
    let recalculate_points = add_float_position(ui, value);

    let mut animate = animation.is_some();

    let initial_animate = animate;

    ui.checkbox(&mut animate, "animate");

    let toggle_animate = animate != initial_animate;

    if toggle_animate {
        toggle_animation(animation, time);
    }

    recalculate_points
}

fn update_animate(v: &mut f32, animation: &Option<Animation>, time: Time<Virtual>) -> bool {
    if let Some(animation) = animation {
        *v = animate(time, animation);
        true
    } else {
        false
    }
}

fn animate(time: Time<Virtual>, animation: &Animation) -> f32 {
    let range = -1.0..=1.0;
    let np = NP as f32;
    let range = (*range.start() * np)..=(*range.end() * np);
    let step = 0.1 * np;
    animation.update_value(time, step, *range.start(), *range.end())
}
