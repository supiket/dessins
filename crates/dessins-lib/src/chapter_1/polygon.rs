use crate::{Segment, Shape, Shapes, NP};
use nannou::prelude::*;
use nannou_egui::egui::Ui;
use ui_controlled_params::UiControlledParams;

#[derive(UiControlledParams)]
#[params(Polygon)]
pub struct ParamsInner {
    #[param(label = "polygon k", range(3..=20))]
    pub k: u32, // # vertices
    #[param(label = "polygon r", length)]
    pub r: f32, // radius of the circle on which the vertices are
    #[param(label = "polygon ad", pi)]
    pub ad: f32, // angle (in radians) of the vector CS with horizontal, where S is the first vertex
}

impl ParamsInner {
    pub fn calculate_shapes(&self) -> Shapes {
        let mut shapes = Shapes::new();
        let mut shape = Shape::new();
        let mut segment = Segment::new();

        for i in 0..self.k {
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

impl Default for Params {
    fn default() -> Self {
        Self {
            inner: ParamsInner {
                k: 3,
                r: NP as f32 * 0.45,
                ad: 0_f32,
            },
            calculate_shapes: Box::new(ParamsInner::calculate_shapes),
            ui_elements: Box::new(ParamsInner::ui_elements),
        }
    }
}
