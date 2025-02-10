use crate::{DesignParams, Model, Shapes};
use nannou::prelude::*;
use nannou_egui::egui::Ui;

pub type UiElements = Box<dyn Fn(&mut NoParamsInner, &mut Ui) -> bool>;

pub struct NoParamsInner();

pub struct NoParams {
    pub inner: NoParamsInner,
    pub calculate_shapes: Box<dyn Fn(&mut NoParamsInner) -> Shapes>,
    pub ui_elements: UiElements,
}

impl NoParamsInner {
    pub fn model(
        self,
        app: &App,
        calculate_shapes: Box<dyn Fn(&mut NoParamsInner) -> Shapes>,
    ) -> Model {
        let params = DesignParams::Shape(NoParams {
            inner: self,
            calculate_shapes,
            ui_elements: Box::new(Self::no_ui_elements),
        });

        crate::model(params, app)
    }

    pub fn no_ui_elements(_inner: &mut NoParamsInner, _ui: &mut Ui) -> bool {
        false
    }
}
