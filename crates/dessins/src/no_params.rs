use crate::Shapes;
use nannou::prelude::*;

pub type UiElements = Box<dyn Fn(&mut NoParamsInner, &mut egui::Ui) -> bool + Send + Sync>;

pub struct NoParamsInner();

pub struct NoParams {
    pub inner: NoParamsInner,
    pub calculate_shapes: Box<dyn Fn(&mut NoParamsInner) -> Shapes + Send + Sync>,
    pub ui_elements: UiElements,
}

impl NoParamsInner {
    pub fn no_ui_elements(&mut self, _ui: &mut egui::Ui) -> bool {
        false
    }
}
