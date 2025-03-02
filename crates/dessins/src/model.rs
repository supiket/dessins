use crate::params::DesignParams;
use crate::shapes::{draw_segment, Shapes};
use bevy_egui::EguiContexts;
use nannou::prelude::*;

#[derive(Resource)]
pub struct Model {
    params: DesignParams,
    points: Shapes,
    pub color: Color,
}

impl Model {
    pub fn new(params: DesignParams) -> Self {
        Self {
            params,
            points: Shapes::new_non_empty(),
            color: Color::srgb(random(), random(), random()),
        }
    }

    pub fn initialized(&self) -> bool {
        self.points != Shapes::new_non_empty()
    }

    pub fn calculate_shapes(&mut self) {
        self.points = match &mut self.params {
            DesignParams::Polygon(params) => params.calculate_shapes(),
            DesignParams::Star(params) => params.calculate_shapes(),
            DesignParams::Composition1(params) => params.calculate_shapes(),
            DesignParams::Composition2(params) => params.calculate_shapes(),
            DesignParams::Jolygon(params) => params.calculate_shapes(),
        };
    }

    pub fn draw_points(&self, draw: Single<&Draw>) {
        self.points.iter().for_each(|shape| {
            shape
                .iter()
                .for_each(|segment| draw_segment(&draw, self.color, segment))
        });
    }

    pub fn change_design(&mut self, params: DesignParams) {
        self.params = params;
    }

    pub fn control_params(&mut self, egui_ctx: EguiContexts) -> (bool, Option<Color>) {
        // TODO: add buttons for different designs

        self.params.control(egui_ctx)
    }
}
