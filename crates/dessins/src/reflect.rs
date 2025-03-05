use crate::{
    animation::AnimationState,
    meta::{ParamMeta, ParamsMeta, RangeStep},
    ui::{add_float_with_label, ui_color},
};
use bevy_reflect::{Reflect, TypeInfo};
use nannou::prelude::*;

pub struct FieldControl {
    changed: bool,
    toggle_animate: bool,
}

pub trait ControllableParams: Reflect + GetField {
    fn set_meta(&mut self, path: &str);

    fn get_meta(&self) -> Option<ParamsMeta> {
        self.get_field::<Option<ParamsMeta>>("meta").cloned()?
    }

    fn toggle_animation_state(&mut self, field_key: &str) {
        let meta = self
            .get_field_mut::<Option<ParamsMeta>>("meta")
            .expect("field 'meta' must exist in all dessins")
            .as_mut()
            .expect("dessin 'meta' must be some");
        let ParamMeta { animation, subtype } = meta
            .get_mut(field_key)
            .expect("key must exist in dessin 'meta'");
        *animation = match animation {
            Some(_) => None,
            None => {
                // TODO: add to ParamMeta
                let RangeStep { range, step } = subtype.get_range_step();
                let freq = step.clone();
                Some(AnimationState::new(freq, *range.start(), *range.end()))
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

pub fn control_reflect<T: ControllableParams>(
    data: &mut T,
    ctx: &mut egui::Context,
) -> anyhow::Result<(bool, Option<Color>)> {
    let type_name = std::any::type_name::<T>();
    let meta = extract_params_meta(data);
    let mut changed = false;
    let mut color = None;
    let mut keys_to_toggle_animation_state = vec![];

    egui::SidePanel::left("params").show(ctx, |ui| {
        if let Some(new_color) = ui_color(ui) {
            color = Some(new_color);
        }

        for field_name in get_field_names(data) {
            if data.get_field_mut::<ParamsMeta>(&field_name).is_some() {
                continue;
            } else if data
                .get_field_mut::<Option<ParamsMeta>>(&field_name)
                .is_some()
            {
                continue;
            } else if let Some(v) = data.get_field_mut::<f32>(&field_name) {
                let key = format!("{}.{}", type_name, field_name);
                let field_meta = meta.get(key.as_str()).unwrap();

                let FieldControl {
                    changed: field_changed,
                    toggle_animate,
                } = control_f32_field(v, ui, &field_name, &field_meta);

                if toggle_animate {
                    keys_to_toggle_animation_state.push(key);
                }

                changed |= field_changed;
            } else {
                todo!("unsupported field type: {field_name}");
            }
        }
    });

    for key in keys_to_toggle_animation_state.iter() {
        data.toggle_animation_state(&key);
    }

    Ok((changed, color))
}

pub fn extract_params_meta<T: ControllableParams>(data: &mut T) -> ParamsMeta {
    let type_path = data.reflect_type_path().to_string();

    if data.get_meta().is_none() {
        data.set_meta(&type_path);
    }

    data.get_meta().expect("has to be some")
}

pub fn control_f32_field(
    v: &mut f32,
    ui: &mut egui::Ui,
    name: &str,
    meta: &ParamMeta,
) -> FieldControl {
    let mut field_control = f32_field_ui(v, ui, &name, meta);

    field_control.changed |= f32_field_animate(v, meta);

    field_control
}

pub fn f32_field_ui(v: &mut f32, ui: &mut egui::Ui, name: &str, meta: &ParamMeta) -> FieldControl {
    let ParamMeta { animation, subtype } = meta;
    let changed = add_float_with_label(ui, &name, v, &subtype);

    let mut animate = animation.is_some();
    let initial_animate = animate.clone();

    ui.checkbox(&mut animate, "animate");

    let toggle_animate = animate != initial_animate;

    FieldControl {
        changed,
        toggle_animate,
    }
}

pub fn f32_field_animate(v: &mut f32, meta: &ParamMeta) -> bool {
    if let Some(animation_state) = &meta.animation {
        *v = animation_state.animate_float(&meta.subtype);
        true
    } else {
        false
    }
}
