use super::u32::U32;
use crate::adjustable_variable::{AdjustableVariable, UpdateVariableParams};
use bevy::reflect::Reflect;
use std::ops::RangeInclusive;

#[derive(Clone, Debug, PartialEq, Reflect)]
pub struct VecU32 {
    value: Vec<U32>,
}

impl VecU32 {
    pub fn new(value: Vec<u32>, range: RangeInclusive<u32>) -> Self {
        let value = value
            .into_iter()
            .map(|v| U32::new(v, range.clone()))
            .collect();
        Self { value }
    }

    pub fn get_value(&self) -> &Vec<U32> {
        &self.value
    }

    pub fn get_value_mut(&mut self) -> &mut Vec<U32> {
        &mut self.value
    }

    pub fn set_value(&mut self, value: Vec<U32>) {
        self.value = value;
    }
}

impl AdjustableVariable for VecU32 {
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
