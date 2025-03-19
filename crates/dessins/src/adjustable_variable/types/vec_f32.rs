use super::f32::{F32Variant, F32};
use crate::adjustable_variable::{AdjustableVariable, UpdateVariableParams};
use bevy::reflect::Reflect;

#[derive(Clone, Debug, PartialEq, Reflect)]
pub struct VecF32 {
    value: Vec<F32>,
    variant: F32Variant,
}

impl VecF32 {
    pub fn new(value: Vec<f32>, variant: F32Variant) -> Self {
        let value = value
            .into_iter()
            .map(|v| F32::new(v, variant.clone()))
            .collect();
        Self { value, variant }
    }

    pub fn get_value(&self) -> &Vec<F32> {
        &self.value
    }

    pub fn get_value_mut(&mut self) -> &mut Vec<F32> {
        &mut self.value
    }

    pub fn set_value(&mut self, value: Vec<F32>) {
        self.value = value;
    }
}

impl AdjustableVariable for VecF32 {
    fn update(&mut self, params: UpdateVariableParams) -> bool {
        let UpdateVariableParams { ui, name, time } = params;

        let mut changed = false;
        for (index, value) in self.value.iter_mut().enumerate() {
            changed |= value.update(UpdateVariableParams {
                ui,
                name: format!("{}[{}]", name, index).to_string(),
                time,
            })
        }
        changed
    }
}
