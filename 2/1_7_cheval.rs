use common::{
    chapter_2::{self, Action, DesignShape, Model, HORSE},
    Shape, Shapes, NP,
};
use nannou::prelude::*;

const N: i32 = 4;

fn model(app: &App) -> Model {
    chapter_2::model(Box::new(calculate_shapes), app)
}

fn calculate_shapes() -> Shapes {
    let mut shapes = vec![];

    for i in -{ N }..=N {
        for j in -abs(i)..=abs(i) {
            shapes.push(calculate_shape(i, j));
        }
    }

    shapes
}

fn calculate_shape(i: i32, j: i32) -> Shape {
    let mut shape = vec![vec![]];

    let mut horse = DesignShape::new(HORSE);

    while let Action::Continue(read_point, newline) = horse.calculate_point() {
        if newline {
            shape.push(vec![])
        }

        let xx = (read_point.x + j as f32 * 20.0 - 20.0) / 100.0;
        let yy = (read_point.y + i as f32 * 20.0 - 20.0) / 100.0;
        let x = xx * 0.7 * NP as f32;
        let y = yy * 0.7 * NP as f32;
        let point = pt2(x, y);

        shape[horse.line_index].push(point);
    }

    shape
}

fn main() {
    nannou::app(model).update(chapter_2::update).run();
}
