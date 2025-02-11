use crate::{DesignParams, Model, Shapes};
use nannou::prelude::*;

pub type UiElements = Box<dyn Fn(&mut NoParamsInner, &mut egui::Ui) -> bool + Send + Sync>;

pub struct NoParamsInner();

pub struct NoParams {
    pub inner: NoParamsInner,
    pub calculate_shapes: Box<dyn Fn(&mut NoParamsInner) -> Shapes + Send + Sync>,
    pub ui_elements: UiElements,
}

impl NoParamsInner {
    pub fn model(
        self,
        app: &App,
        calculate_shapes: Box<dyn Fn(&mut NoParamsInner) -> Shapes + Send + Sync>,
    ) -> Model {
        let params = DesignParams::Shape(NoParams {
            inner: self,
            calculate_shapes,
            ui_elements: Box::new(Self::no_ui_elements),
        });

        crate::model(params, app)
    }

    pub fn no_ui_elements(_inner: &mut NoParamsInner, _ui: &mut egui::Ui) -> bool {
        false
    }
}
