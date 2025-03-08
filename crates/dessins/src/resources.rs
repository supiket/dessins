use crate::params::{DesignController, DessinVariant};
use crate::shapes::{draw_segment, Shapes};
use bevy_egui::EguiContexts;
use nannou::prelude::*;

#[derive(Resource)]
pub struct Model {
    dessin: DesignController,
    points: Shapes,
    // TODO: animate
    pub color: Color,
}

impl Model {
    pub fn new(variant: DessinVariant) -> Self {
        Self {
            dessin: DesignController {
                variant,
                variables: variant.get_variables(),
            },
            points: Shapes::new_non_empty(),
            color: Color::srgb(random(), random(), random()),
        }
    }

    pub fn initialized(&self) -> bool {
        self.points != Shapes::new_non_empty()
    }

    pub fn calculate_shapes(&mut self) {
        self.points = self.dessin.variables.calculate_shapes();
    }

    pub fn draw_points(&self, draw: Single<&Draw>) {
        self.points.iter().for_each(|shape| {
            shape
                .iter()
                .for_each(|segment| draw_segment(&draw, self.color, segment))
        });
    }

    pub fn change_design(&mut self, variant: DessinVariant) {
        self.dessin.variables = variant.get_variables();
    }

    pub fn control_variables(
        &mut self,
        mut egui_ctx: EguiContexts,
        time: Time<Virtual>,
    ) -> (bool, Option<Color>) {
        let ctx = egui_ctx.ctx_mut();

        let mut changed = false;
        changed |= self.dessin.control(ctx);
        let control_res = self.dessin.variables.control(ctx, time);
        changed |= control_res.0;

        (changed, control_res.1)
    }
}
