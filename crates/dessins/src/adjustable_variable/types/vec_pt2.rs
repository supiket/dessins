use super::pt2::Pt2;
use crate::adjustable_variable::{AdjustableVariable, UpdateVariableParams};
use bevy::reflect::Reflect;
use nannou::prelude::*;

#[derive(Clone, Debug, PartialEq, Reflect)]
pub struct VecPt2 {
    value: Vec<Pt2>,
}

impl VecPt2 {
    pub fn new(value: Vec<Point2>) -> Self {
        let value = value.into_iter().map(Pt2::new).collect();
        Self { value }
    }

    pub fn get_value(&self) -> &Vec<Pt2> {
        &self.value
    }

    pub fn get_value_mut(&mut self) -> &mut Vec<Pt2> {
        &mut self.value
    }

    pub fn set_value(&mut self, value: Vec<Pt2>) {
        self.value = value;
    }
}

impl AdjustableVariable for VecPt2 {
    fn update(&mut self, params: UpdateVariableParams) -> bool {
        let UpdateVariableParams {
            ui,
            osc_ctx,
            name,
            time,
        } = params;

        let mut changed = false;
        for (index, value) in self.value.iter_mut().enumerate() {
            changed |= value.update(UpdateVariableParams {
                ui,
                osc_ctx,
                name: format!("{}[{}]", name, index).to_string(),
                time,
            })
        }
        changed
    }
}
