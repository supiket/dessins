use crate::{
    adjustable_variable::{
        types::{Context, ExpressionF32, Pt2, VecF32, VecPt2, VecU32, F32, U32},
        AdjustableVariable, UpdateVariableParams,
    },
    ui::ui_color,
};
use bevy_reflect::{Reflect, TypeInfo};
use nannou::prelude::*;

pub trait AdjustableDessin: Reflect + GetField {
    fn update_dessin(
        &mut self,
        ui: &mut egui::Ui,
        osc_ctx: &Context,
        time: Time<Virtual>,
    ) -> (bool, Option<Color>)
    where
        Self: Sized,
    {
        let mut color = None;

        if let Some(new_color) = ui_color(ui) {
            color = Some(new_color);
        }

        ui.separator();

        let changed = self.update_variables(ui, osc_ctx, time);

        (changed, color)
    }

    fn update_variables(
        &mut self,
        ui: &mut egui::Ui,
        osc_ctx: &Context,
        time: Time<Virtual>,
    ) -> bool
    where
        Self: Sized,
    {
        update_from_reflect(self, ui, osc_ctx, time)
    }
}

fn get_field_names<T: AdjustableDessin>(data: &T) -> Vec<&'static str> {
    let type_info = data.as_reflect().reflect_type_info();

    if let TypeInfo::Struct(info) = type_info {
        info.field_names().to_vec()
    } else {
        panic!("cannot get field names for struct");
    }
}

pub fn update_from_reflect<T: AdjustableDessin>(
    data: &mut T,
    ui: &mut egui::Ui,
    osc_ctx: &Context,
    time: Time<Virtual>,
) -> bool {
    let mut changed = false;

    for field_name in get_field_names(data) {
        let params = UpdateVariableParams {
            ui,
            osc_ctx,
            time,
            name: field_name.to_string(),
        };
        if let Some(inner) = data.get_field_mut::<U32>(field_name) {
            changed |= inner.update(params);
        } else if let Some(inner) = data.get_field_mut::<F32>(field_name) {
            changed |= inner.update(params);
        } else if let Some(inner) = data.get_field_mut::<ExpressionF32>(field_name) {
            changed |= inner.update(params);
        } else if let Some(inner) = data.get_field_mut::<Pt2>(field_name) {
            changed |= inner.update(params);
        } else if let Some(inner) = data.get_field_mut::<VecF32>(field_name) {
            changed |= inner.update(params);
        } else if let Some(inner) = data.get_field_mut::<VecU32>(field_name) {
            changed |= inner.update(params);
        } else if let Some(inner) = data.get_field_mut::<VecPt2>(field_name) {
            changed |= inner.update(params);
        } else {
            let type_name = std::any::type_name::<T>();
            todo!("unsupported field type: {field_name} in {type_name}");
        }

        ui.separator();
    }

    changed
}
