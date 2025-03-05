use crate::{
    meta::{ParamMeta, ParamsMeta},
    reflect::ControllableParams,
    shapes::{Segment, Shape, Shapes, NP},
};
use nannou::prelude::*;

#[derive(Clone, Debug, PartialEq, Reflect)]
#[reflect(Default)]
pub struct Star {
    pub k: f32,  // # vertices
    pub h: f32,  // # vertices to skip (clockwise) before connecting two dots
    pub r: f32,  // radius of the circle C on which the vertices are
    pub ad: f32, // angle (in radians) of the vector CS with horizontal, where S is the first vertex
    pub meta: Option<ParamsMeta>,
}

impl Star {
    pub fn calculate_shapes(&mut self) -> Shapes {
        let mut shapes = Shapes::new();
        let mut shape = Shape::new();
        let mut segment = Segment::new();

        for i in 0..self.k as u32 {
            let point = self.calculate_point(i);
            segment.push(point);
        }

        segment.push(segment[0]);

        shape.push(segment);
        shapes.push(shape);
        shapes
    }

    pub fn calculate_point(&self, i: u32) -> Point2 {
        let angle = (2.0 * i as f32 * self.h as f32 * PI) / self.k as f32 + self.ad;
        let x = self.r * angle.cos();
        let y = self.r * angle.sin();
        pt2(x, y)
    }
}

impl ControllableParams for Star {
    fn set_meta(&mut self, path: &str) {
        self.meta = Some(ParamsMeta(
            [
                (format!("{}.k", path), ParamMeta::new(5.0..=100.0)),
                (format!("{}.h", path), ParamMeta::new(3.0..=5.0)),
                (format!("{}.r", path), ParamMeta::new_length()),
                (format!("{}.ad", path), ParamMeta::new_angle()),
            ]
            .into_iter()
            .collect(),
        ));
    }
}

impl Default for Star {
    fn default() -> Self {
        Self {
            k: 5.0,
            h: 3.0,
            r: NP as f32 * 0.45,
            ad: PI / 2.0,
            meta: None,
        }
    }
}
