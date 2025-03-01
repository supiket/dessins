use super::{
    polygon::{self},
    star::{self},
};
use crate::{Segment, Shape, Shapes, NP};

pub struct Params {
    pub polygon: polygon::Params,
    pub star: star::Params,
    pub n: u32,  // # stars
    pub rr: f32, // reduction coefficient from one star to the next & the distance between the center of the spiral and the center of successive stars
}

impl Params {
    pub fn calculate_shapes(&mut self) -> Shapes {
        let mut shapes = Shapes::default();
        let mut shape = Shape::new();

        let mut polygon = self.polygon.clone();
        let mut star = self.star.clone();

        for i in 0..self.n {
            let r2 = self.polygon.r * self.rr.powi(i as i32);
            let r3 = self.star.r * self.rr.powi(i as i32);

            polygon.r = r2;
            let polygon_point = polygon.calculate_point(i);

            let mut segment = Segment::new();

            for j in 0..self.star.k {
                star.r = r3;
                let star_point = star.calculate_point(j);
                let point = star_point + polygon_point;
                segment.push(point);
            }

            segment.push(segment[0]);

            shape.push(segment);
        }

        shapes.push(shape);

        shapes
    }
}

impl Default for Params {
    fn default() -> Self {
        Self {
            n: 32,
            rr: 0.9,
            polygon: polygon::Params {
                k: 8,
                r: NP as f32 * 0.36,
                ad: 0_f32,
                ..Default::default()
            },
            star: star::Params {
                k: 16,
                h: 5,
                r: NP as f32 * 0.14,
                ad: 0_f32,
            },
        }
    }
}
