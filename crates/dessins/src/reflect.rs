use crate::{
    meta::{FieldMeta, FieldsMeta, ParamMeta},
    ui::{add_float_with_label, ui_color},
};
use bevy_reflect::ReflectMut;
use nannou::prelude::*;

pub struct FieldControl {
    changed: bool,
    toggle_animate: bool,
}

pub fn control_reflect<T: Reflect + ParamMeta>(
    data: &mut T,
    ctx: &mut egui::Context,
) -> anyhow::Result<(bool, Option<Color>)> {
    let type_name = std::any::type_name::<T>();
    let meta = extract_fields_meta(data);
    let mut changed = false;
    let mut color = None;
    let mut keys_to_toggle_animation_state = vec![];

    if let ReflectMut::Struct(s) = data.reflect_mut() {
        egui::SidePanel::left("params").show(ctx, |ui| {
            if let Some(new_color) = ui_color(ui) {
                color = Some(new_color);
            }

            for i in 0..s.field_len() {
                // TODO: handle error
                let (
                    key,
                    FieldControl {
                        changed: field_changed,
                        toggle_animate,
                    },
                ) = control_field(s, ui, type_name, i, &meta).unwrap();

                if toggle_animate {
                    keys_to_toggle_animation_state.push(key);
                }

                changed |= field_changed;
            }
        });

        for key in keys_to_toggle_animation_state.iter() {
            data.toggle_field_animation_state(&key).unwrap();
        }
    } else {
        return Err(anyhow::anyhow!("data is not a struct"));
    }

    Ok((changed, color))
}

pub fn extract_fields_meta<T: Reflect + ParamMeta>(data: &mut T) -> FieldsMeta {
    let type_path = data.reflect_type_path().to_string();

    if data.get_fields_meta().is_none() {
        data.set_fields_meta(&type_path);
    }

    data.get_fields_meta().expect("has to be some")
}

pub fn control_field<S: bevy_reflect::Struct + ?Sized>(
    s: &mut S,
    ui: &mut egui::Ui,
    s_type_name: &str,
    field_index: usize,
    fields_meta: &FieldsMeta,
) -> anyhow::Result<(String, FieldControl)> {
    if let Some(field_name) = s.name_at(field_index) {
        let field_name = field_name.to_owned();
        let key = format!("{}.{}", s_type_name, field_name);
        let field_meta = fields_meta.get(key.as_str()).unwrap();

        if let Some(field) = s.field_mut(&field_name) {
            if let Some(v) = field.try_downcast_mut::<f32>() {
                Ok((key, control_f32_field(v, ui, &field_name, &field_meta)))
            } else {
                Err(anyhow::anyhow!(format!(
                    "unsupported field type: {}",
                    field_name
                )))
            }
        } else {
            Err(anyhow::anyhow!(format!(
                "field does not exist: {}",
                field_name
            )))
        }
    } else {
        Err(anyhow::anyhow!(format!(
            "field at index does not exist: {}",
            field_index
        )))
    }
}

pub fn control_f32_field(
    v: &mut f32,
    ui: &mut egui::Ui,
    name: &str,
    meta: &FieldMeta,
) -> FieldControl {
    let mut field_control = f32_field_ui(v, ui, &name, meta);

    field_control.changed |= f32_field_animate(v, meta);

    field_control
}

pub fn f32_field_ui(v: &mut f32, ui: &mut egui::Ui, name: &str, meta: &FieldMeta) -> FieldControl {
    let FieldMeta { animation, subtype } = meta;
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

pub fn f32_field_animate(v: &mut f32, meta: &FieldMeta) -> bool {
    if let Some(animation_state) = &meta.animation {
        *v = animation_state.animate_float(&meta.subtype);
        true
    } else {
        false
    }
}
