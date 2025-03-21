use crate::{
    adjustable_variable::types::Context,
    dessin_with_variables::{DessinVariant, DessinWithVariables},
    osc::Osc,
    shapes::{Shapes, WEIGHT},
};
use bevy_egui::EguiContexts;
use nannou::prelude::*;

#[derive(Resource)]
pub struct Model {
    active_dessin: DessinWithVariables,
    osc: Osc,
    points: Shapes,
    // TODO: animate
    pub color: Color,
}

impl Model {
    pub fn new(variant: DessinVariant) -> Self {
        Self {
            active_dessin: DessinWithVariables {
                variant,
                variables: variant.get_variables(),
            },
            osc: Osc::default(),
            points: Shapes::new_non_empty(),
            color: Color::srgb(random(), random(), random()),
        }
    }

    pub fn initialized(&self) -> bool {
        self.points != Shapes::new_non_empty()
    }

    pub fn calculate_shapes(&mut self) {
        self.points = self.active_dessin.variables.calculate_shapes();
    }

    pub fn draw_points(&self, draw: Single<&Draw>) {
        self.points.iter().for_each(|shape| {
            shape.iter().for_each(|segment| {
                let points_colored = segment.iter().copied().map(|point| (point, self.color));
                draw.polyline()
                    .weight(WEIGHT)
                    .points_colored(points_colored);
            })
        });
    }

    pub fn update_active_dessin(
        &mut self,
        mut egui_ctx: EguiContexts,
        time: Time<Virtual>,
    ) -> (bool, Option<Color>) {
        let ctx = egui_ctx.ctx_mut();

        let mut changed = false;
        changed |= self.active_dessin.update(ctx);

        let mut color_changed = None;
        let mut osc_ctx = Context::new(Default::default());

        egui::SidePanel::left("osc and variables").show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                osc_ctx = self.osc.update(ui);
                ui.separator();
                let res = self.active_dessin.variables.update(ui, &osc_ctx, time);
                changed |= res.0;
                color_changed = res.1;
            });
        });

        (changed, color_changed)
    }
}
