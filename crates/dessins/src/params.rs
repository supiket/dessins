use crate::{chapter_1, reflect::control_reflect, shapes::Shapes};
use nannou::prelude::*;

pub struct DesignController {
    pub selected: DesignVariant,
    pub params: DesignParams,
}

#[derive(PartialEq, Reflect)]
pub enum DesignParams {
    Polygon(chapter_1::Polygon),
    Star(chapter_1::Star),
    Composition1(chapter_1::Composition1),
    Composition2(chapter_1::Composition2),
    Jolygon(chapter_1::Jolygon),
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum DesignVariant {
    Polygon,
    Star,
    Composition1,
    Composition2,
    Jolygon,
}

impl DesignController {
    pub fn control(&mut self, ctx: &mut egui::Context) -> bool {
        let mut changed = false;

        egui::TopBottomPanel::top("dessins").show(ctx, |ui| {
            ui.horizontal(|ui| {
                changed |= ui
                    .selectable_value(&mut self.selected, DesignVariant::Polygon, "polygon")
                    .changed();
                changed |= ui
                    .selectable_value(&mut self.selected, DesignVariant::Star, "star")
                    .changed();
                changed |= ui
                    .selectable_value(
                        &mut self.selected,
                        DesignVariant::Composition1,
                        "composition 1",
                    )
                    .changed();
                changed |= ui
                    .selectable_value(
                        &mut self.selected,
                        DesignVariant::Composition2,
                        "composition 2",
                    )
                    .changed();
                changed |= ui
                    .selectable_value(&mut self.selected, DesignVariant::Jolygon, "jolygon")
                    .changed();
            });
        });

        if changed {
            self.params = self.selected.get_params();
        }

        changed
    }
}

impl DesignVariant {
    pub fn get_params(&self) -> DesignParams {
        match self {
            Self::Polygon => DesignParams::Polygon(chapter_1::Polygon::default()),
            Self::Star => DesignParams::Star(chapter_1::Star::default()),
            Self::Composition1 => DesignParams::Composition1(chapter_1::Composition1::default()),
            Self::Composition2 => DesignParams::Composition2(chapter_1::Composition2::default()),
            Self::Jolygon => DesignParams::Jolygon(chapter_1::Jolygon::default()),
        }
    }
}

impl DesignParams {
    pub fn calculate_shapes(&mut self) -> Shapes {
        match self {
            DesignParams::Polygon(params) => params.calculate_shapes(),
            DesignParams::Star(params) => params.calculate_shapes(),
            DesignParams::Composition1(params) => params.calculate_shapes(),
            DesignParams::Composition2(params) => params.calculate_shapes(),
            DesignParams::Jolygon(params) => params.calculate_shapes(),
        }
    }

    pub fn control(&mut self, ctx: &mut egui::Context) -> (bool, Option<Color>) {
        match self {
            DesignParams::Polygon(params) => control_reflect(params, ctx),
            DesignParams::Star(params) => control_reflect(params, ctx),
            DesignParams::Composition1(params) => control_reflect(params, ctx),
            DesignParams::Composition2(params) => control_reflect(params, ctx),
            DesignParams::Jolygon(params) => control_reflect(params, ctx),
        }
    }
}
