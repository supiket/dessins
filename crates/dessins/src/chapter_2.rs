use crate::{reflect::ControllableParams, shapes::Shapes};
use nannou::prelude::*;
use raw_shape::*;
use shape_program::*;

mod raw_shape;
mod shape_program;

#[derive(Clone, Debug, PartialEq, Reflect)]
#[reflect(Default)]
pub struct Params {
    #[reflect(ignore)]
    pub raw_shape: RawShape,
    #[reflect(ignore)]
    pub shape_program: ShapeProgram,
}

impl Params {
    pub fn calculate_shapes(&mut self) -> Shapes {
        let mut design_shape = DesignShape::new(&self.raw_shape);
        self.shape_program.calculate_shapes(&mut design_shape)
    }
}

impl ControllableParams for Params {
    fn control(&mut self, ui: &mut egui::Ui, _time: Time<Virtual>) -> bool {
        let mut changed = false;
        changed |= self.raw_shape.control(ui);
        changed |= self.shape_program.control(ui);
        changed
    }
}

impl Default for Params {
    fn default() -> Self {
        Self {
            raw_shape: RawShape::Horse,
            shape_program: ShapeProgram::Program1,
        }
    }
}
