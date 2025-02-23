use crate::{Segment, Shape, Shapes, NP};
use nannou::prelude::*;
use ui_controlled_params::UiControlledParams;

#[derive(UiControlledParams)]
#[params(Jolygon)]
pub struct ParamsInner {
    #[param(range(1..=2500))]
    pub k: u32, // # segments
    #[param(pi)]
    pub an: f32, // angle of two consecutive segments
    #[param(range(0.9..=1.0))]
    pub ra: f32, // ratio of the lengths of two consecutive segments
    #[param(pi)]
    pub aa: f32, // angle of the first segment with horizontal
    #[param(length)]
    pub rr: f32, // length of the first segment
}

impl ParamsInner {
    pub fn calculate_shapes(&mut self) -> Shapes {
        let mut shapes = Shapes::new();
        let mut shape = Shape::new();
        let mut segment = Segment::new();

        let mut current_length = self.rr;
        let mut current_pos = pt2(0.0, 0.0);
        segment.push(current_pos);

        let mut min_x = 0.0;
        let mut max_x = 0.0;
        let mut min_y = 0.0;
        let mut max_y = 0.0;

        for i in 0..self.k {
            let angle = self.aa + i as f32 * self.an;

            let dx = current_length * angle.cos();
            let dy = current_length * angle.sin();
            let d = pt2(dx, dy);
            let point = current_pos + d;

            // update bounds
            min_x = min_x.min(point.x);
            max_x = max_x.max(point.x);
            min_y = min_y.min(point.y);
            max_y = max_y.max(point.y);

            segment.push(point);
            current_pos = point;
            current_length *= self.ra;
        }

        // calculate center offset
        let center_offset_x = (min_x + max_x) / 2.0;
        let center_offset_y = (min_y + max_y) / 2.0;

        // make segments centered
        for point in &mut segment {
            *point -= pt2(center_offset_x, center_offset_y);
        }

        shape.push(segment);
        shapes.push(shape);

        shapes
    }
}

impl Default for Params {
    fn default() -> Self {
        Self {
            inner: ParamsInner {
                k: 200,
                an: 15.0 * PI / 31.0,
                ra: 0.98,
                aa: 0_f32,
                rr: 0.80 * NP as f32,
            },
            calculate_shapes: Box::new(ParamsInner::calculate_shapes),
            ui_elements: Box::new(ParamsInner::ui_elements),
        }
    }
}
