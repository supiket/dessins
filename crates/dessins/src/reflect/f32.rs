use super::ControlAction;
use crate::meta::{ParamMeta, ParamType};
use nannou::prelude::*;

pub fn control(v: &mut f32, ui: &mut egui::Ui, name: &str, meta: &ParamMeta) -> ControlAction {
    let mut field_control = control_ui(v, ui, name, meta);

    field_control.changed |= control_animate(v, meta);

    field_control
}

pub fn control_ui(v: &mut f32, ui: &mut egui::Ui, name: &str, meta: &ParamMeta) -> ControlAction {
    let ParamMeta {
        animation,
        param_type,
    } = meta;
    let ParamType::F32(f32_type) = param_type else {
        panic!("expected ParamType::F32")
    };
    let changed = f32_type.add_with_label(ui, name, v);

    let mut animate = animation.is_some();
    let initial_animate = animate;

    ui.checkbox(&mut animate, "animate");

    let toggle_animate = animate != initial_animate;

    ControlAction {
        changed,
        toggle_animate,
    }
}

pub fn control_animate(v: &mut f32, meta: &ParamMeta) -> bool {
    if let Some(animation_state) = &meta.animation {
        let ParamType::F32(f32_type) = &meta.param_type else {
            panic!("expected ParamType::F32")
        };
        *v = f32_type.animate(animation_state);
        true
    } else {
        false
    }
}
