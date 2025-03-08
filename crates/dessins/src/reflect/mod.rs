use crate::{
    meta::{expression_f32::ExpressionF32, f32::F32},
    ui::ui_color,
};
use bevy_reflect::{Reflect, TypeInfo};
use nannou::prelude::*;

pub struct ControlAction {
    pub changed: bool,
    pub toggle_animate: bool,
}

pub trait ControllableParams: Reflect + GetField {
    fn control_params(
        &mut self,
        ctx: &mut egui::Context,
        time: Time<Virtual>,
    ) -> (bool, Option<Color>)
    where
        Self: Sized,
    {
        let mut changed = false;
        let mut color = None;

        egui::SidePanel::left("params").show(ctx, |ui| {
            if let Some(new_color) = ui_color(ui) {
                color = Some(new_color);
            }

            changed |= self.control(ui, time)
        });

        (changed, color)
    }

    fn control(&mut self, ui: &mut egui::Ui, time: Time<Virtual>) -> bool
    where
        Self: Sized,
    {
        control_reflect(self, ui, time)
    }
}

pub trait ControllableParam: Reflect {
    fn control(&mut self, ui: &mut egui::Ui, name: &str, time: Time<Virtual>) -> bool;

    fn toggle_animation_state(&mut self, time: Time<Virtual>);
}

fn get_field_names<T: ControllableParams>(data: &T) -> Vec<&'static str> {
    let type_info = data.as_reflect().reflect_type_info();

    if let TypeInfo::Struct(info) = type_info {
        info.field_names().to_vec()
    } else {
        panic!("cannot get field names for struct");
    }
}

pub fn control_reflect<T: ControllableParams>(
    data: &mut T,
    ui: &mut egui::Ui,
    time: Time<Virtual>,
) -> bool {
    let mut changed = false;

    for field_name in get_field_names(data) {
        if let Some(param) = data.get_field_mut::<F32>(field_name) {
            changed |= param.control(ui, field_name, time);
        } else if let Some(param) = data.get_field_mut::<ExpressionF32>(field_name) {
            changed |= param.control(ui, field_name, time);
        } else {
            let type_name = std::any::type_name::<T>();
            todo!("unsupported field type: {field_name} in {type_name}");
        }
    }

    changed
}
