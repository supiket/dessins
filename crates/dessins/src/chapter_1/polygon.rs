use crate::{
    meta::{f32::F32Type, ParamMeta, ParamsMeta},
    reflect::ControllableParams,
    shapes::{Segment, Shape, Shapes, NP},
};
use nannou::prelude::*;

#[derive(Clone, Debug, PartialEq, Reflect)]
#[reflect(Default)]
pub struct Polygon {
    // TODO: reconsider f32, at least discretize behaviour
    pub k: f32,  // # vertices
    pub r: f32,  // radius of the circle on which the vertices are
    pub ad: f32, // angle (in radians) of the vector CS with horizontal, where S is the first vertex,
    pub meta: Option<ParamsMeta>,
}

impl Polygon {
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
        let angle = (2.0 * i as f32 * PI) / self.k as f32 + self.ad;
        let x = self.r * angle.cos();
        let y = self.r * angle.sin();
        pt2(x, y)
    }
}

impl ControllableParams for Polygon {
    fn set_meta(&mut self, path: &str) {
        self.meta = Some(ParamsMeta(
            [
                (
                    format!("{}.k", path),
                    ParamMeta::new_f32_from_range(3.0..=20.0),
                ),
                (format!("{}.r", path), ParamMeta::new_f32(F32Type::Length)),
                (format!("{}.ad", path), ParamMeta::new_f32(F32Type::Angle)),
            ]
            .into_iter()
            .collect(),
        ));
    }
}

impl Default for Polygon {
    fn default() -> Self {
        Self {
            k: 3.0,
            r: NP as f32 * 0.45,
            ad: 0_f32,
            meta: None,
        }
    }
}
