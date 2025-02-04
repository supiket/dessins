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
        for j in 0..2 {
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

        let x = (1.0 - 2.0 * j as f32) * NP as f32 * read_point.x / 80.0 * r;
        let y = NP as f32 * (0.5 - r + read_point.y / 80.0 * r);
        let point = pt2(x, y);

        shape[horse.line_index].push(point);
    }

    shape
}

fn main() {
    nannou::app(model).update(chapter_2::update).run();
}
