use crate::{adjustable_dessin::AdjustableDessin, shapes::Shapes};
use nannou::prelude::*;
use raw_shape_program::*;
use raw_shape_variant::*;

mod raw_shape_program;
mod raw_shape_variant;

#[derive(Clone, Debug, PartialEq, Reflect)]
#[reflect(Default)]
pub struct RawShape {
    #[reflect(ignore)]
    pub shape_variant: RawShapeVariant,
    #[reflect(ignore)]
    pub program_variant: RawShapeProgram,
}

impl RawShape {
    pub fn calculate_shapes(&mut self) -> Shapes {
        let mut raw_shape_decoder = RawShapeDecoder::new(&self.shape_variant);
        self.program_variant
            .calculate_shapes(&mut raw_shape_decoder)
    }
}

impl AdjustableDessin for RawShape {
    fn update_variables(&mut self, ui: &mut egui::Ui, _time: Time<Virtual>) -> bool {
        let mut changed = false;
        changed |= self.shape_variant.update(ui);
        ui.separator();
        changed |= self.program_variant.update(ui);
        ui.separator();
        changed
    }
}

impl Default for RawShape {
    fn default() -> Self {
        Self {
            shape_variant: RawShapeVariant::Horse,
            program_variant: RawShapeProgram::Program1,
        }
    }
}
