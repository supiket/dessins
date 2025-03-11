use nannou::prelude::*;

pub const NP: usize = 480; // # elementary steps, i.e. resolution
pub const WEIGHT: f32 = 1.0; // point weight

#[derive(Debug, PartialEq)]
pub struct Shapes(Vec<Shape>);
pub type Shape = Vec<Segment>;
pub type Segment = Vec<Point2>;

pub fn sign(val: f32) -> f32 {
    if val < 0.0 {
        -1.0
    } else if val == 0.0 {
        val
    } else {
        1.0
    }
}

impl Default for Shapes {
    fn default() -> Self {
        Self::new()
    }
}

impl Shapes {
    pub fn new_non_empty() -> Self {
        Self(vec![vec![vec![Point2::default()]]])
    }

    pub fn new() -> Self {
        Self(Default::default())
    }
}

impl core::ops::Deref for Shapes {
    type Target = Vec<Shape>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl core::ops::DerefMut for Shapes {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
