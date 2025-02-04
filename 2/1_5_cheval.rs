use common::{
    chapter_2::{self, Action, DesignShape, Model, HORSE},
    Shape, Shapes, NP,
};
use nannou::prelude::*;

const N: i32 = 6;

fn model(app: &App) -> Model {
    chapter_2::model(Box::new(calculate_shapes), app)
}

fn calculate_shapes() -> Shapes {
    let mut shapes = vec![];

    for i in 0..N {
        for j in 0..pow(2, i as usize) {
            shapes.push(calculate_shape(i, j));
        }
    }

    shapes
}

fn calculate_shape(i: i32, j: i32) -> Shape {
    let mut shape = vec![vec![]];

    let mut horse = DesignShape::new(HORSE);

    let r = pow(0.5, i as usize);

    while let Action::Continue(read_point, newline) = horse.calculate_point() {
        if newline {
            shape.push(vec![])
        }

        let x = (j as f32 + read_point.x / 40.0) * NP as f32 * r - 0.5 * NP as f32;
        let y = (2.0 - 2.0 * r) * NP as f32 + read_point.y / 40.0 * NP as f32 * r - 0.9 * NP as f32;
        let (x, y) = (x * 0.7, y * 0.7);
        let point = pt2(x, y);

        shape[horse.line_index].push(point);
    }

    shape
}

fn main() {
    nannou::app(model).update(chapter_2::update).run();
}
