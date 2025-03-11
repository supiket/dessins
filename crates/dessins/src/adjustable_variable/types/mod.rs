use super::UpdateVariableParams;
use crate::adjustable_variable::AdjustableVariable;
use bevy::reflect::Reflect;
use expression_f32::ExpressionF32;
use f32::F32;
use pt2::Pt2;
use u32::U32;

pub mod expression_f32;
pub mod f32;
pub mod pt2;
pub mod u32;

#[derive(Clone, Debug, PartialEq, Reflect)]
pub enum VariableType {
    U32(U32),
    F32(F32),
    ExpressionF32(ExpressionF32),
    Point2(Pt2),
}

impl AdjustableVariable for VariableType {
    fn update(&mut self, params: UpdateVariableParams) -> bool {
        match self {
            Self::U32(inner) => inner.update(params),
            Self::F32(inner) => inner.update(params),
            Self::ExpressionF32(inner) => inner.update(params),
            Self::Point2(inner) => inner.update(params),
        }
    }
}
