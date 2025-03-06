use crate::{
    animation::AnimationState,
    meta::{f32::RangeStep, ParamMeta, ParamType, ParamsMeta},
    ui::ui_color,
};
use bevy_reflect::{Reflect, TypeInfo};
use nannou::prelude::*;

mod f32;

pub struct ControlAction {
    changed: bool,
    toggle_animate: bool,
}

pub trait ControllableParams: Reflect + GetField {
    fn control_params(&mut self, ctx: &mut egui::Context) -> (bool, Option<Color>)
    where
        Self: Sized,
    {
        let mut changed = false;
        let mut color = None;

        egui::SidePanel::left("params").show(ctx, |ui| {
            if let Some(new_color) = ui_color(ui) {
                color = Some(new_color);
            }

            changed |= self.control(ui)
        });

        (changed, color)
    }

    fn control(&mut self, ui: &mut egui::Ui) -> bool
    where
        Self: Sized,
    {
        control_reflect(self, ui)
    }

    fn set_meta(&mut self, _path: &str) {}

    fn get_meta(&self) -> Option<ParamsMeta> {
        self.get_field::<Option<ParamsMeta>>("meta").cloned()?
    }

    fn toggle_animation_state(&mut self, field_key: &str) {
        let meta = self
            .get_field_mut::<Option<ParamsMeta>>("meta")
            .expect("field 'meta' must exist in all dessins")
            .as_mut()
            .expect("dessin 'meta' must be some");
        let ParamMeta {
            animation,
            param_type,
        } = meta
            .get_mut(field_key)
            .expect("key must exist in dessin 'meta'");
        *animation = match animation {
            Some(_) => None,
            None => {
                match param_type {
                    ParamType::F32(f32_type) => {
                        // TODO: add to ParamMeta
                        let RangeStep { range, step } = f32_type.get_range_step();
                        let freq = step;
                        Some(AnimationState::new(freq, *range.start(), *range.end()))
                    }
                    ParamType::ExpressionF32(_expression_f32) => {
                        todo!("animate values in the context maps")
                    }
                }
            }
        };
    }
}

fn get_field_names<T: ControllableParams>(data: &T) -> Vec<&'static str> {
    let type_info = data.as_reflect().reflect_type_info();

    if let TypeInfo::Struct(info) = type_info {
        info.field_names().to_vec()
    } else {
        panic!("cannot get field names for struct");
    }
}

pub fn control_reflect<T: ControllableParams>(data: &mut T, ui: &mut egui::Ui) -> bool {
    let type_name = std::any::type_name::<T>();
    let meta = extract_params_meta(data);
    let mut changed = false;
    let mut keys_to_toggle_animation_state = vec![];

    for field_name in get_field_names(data) {
        if data
            .get_field_mut::<Option<ParamsMeta>>(field_name)
            .is_some()
        {
            continue;
        } else if let Some(v) = data.get_field_mut::<f32>(field_name) {
            let key = format!("{}.{}", type_name, field_name);
            let field_meta = meta.get(key.as_str()).unwrap();

            let ControlAction {
                changed: field_changed,
                toggle_animate,
            } = f32::control(v, ui, field_name, field_meta);

            if toggle_animate {
                keys_to_toggle_animation_state.push(key);
            }

            changed |= field_changed;
        } else {
            todo!("unsupported field type: {field_name}");
        }
    }

    for key in keys_to_toggle_animation_state.iter() {
        data.toggle_animation_state(key);
    }

    changed
}

pub fn extract_params_meta<T: ControllableParams>(data: &mut T) -> ParamsMeta {
    let type_path = data.reflect_type_path().to_string();

    if data.get_meta().is_none() {
        data.set_meta(&type_path);
    }

    data.get_meta().expect("has to be some")
}
