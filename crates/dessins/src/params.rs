use crate::{chapter_1, reflect::control_reflect};
use bevy_egui::EguiContexts;
use nannou::prelude::*;

#[derive(Reflect)]
pub enum DesignParams {
    Polygon(chapter_1::Polygon),
    Star(chapter_1::Star),
    Composition1(chapter_1::Composition1),
    Composition2(chapter_1::Composition2),
    Jolygon(chapter_1::Jolygon),
}

impl DesignParams {
    pub fn control(&mut self, mut egui_ctx: EguiContexts) -> (bool, Option<Color>) {
        let mut ctx = egui_ctx.ctx_mut();

        // TODO: generate from reflect
        let res = match self {
            DesignParams::Polygon(params) => control_reflect(params, &mut ctx),
            DesignParams::Star(params) => control_reflect(params, &mut ctx),
            DesignParams::Composition1(params) => control_reflect(params, &mut ctx),
            DesignParams::Composition2(params) => control_reflect(params, &mut ctx),
            DesignParams::Jolygon(params) => control_reflect(params, &mut ctx),
        };
        // TODO: handle error
        res.unwrap()
    }
}
