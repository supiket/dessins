use common::{
    chapter_2::{self, Action, DesignShape, Model, LION},
    Shape, Shapes, NP,
};
use nannou::prelude::*;

fn model(app: &App) -> Model {
    chapter_2::model(Box::new(calculate_shapes), app)
}

fn calculate_shapes() -> Shapes {
    let mut shapes = vec![];

    for i in 0..5 {
        for j in 0..3 {
            shapes.push(calculate_shape(i, j));
        }
    }

    shapes
}

fn calculate_shape(i: i32, j: i32) -> Shape {
    let mut shape = vec![vec![]];

    let mut lion = DesignShape::new(LION);

    while let Action::Continue(read_point, newline) = lion.calculate_point() {
        if newline {
            shape.push(vec![])
        }

        let x = NP as f32
            * (-18.0 + (1.0 - 2.0 * (i % 2) as f32) * (7.0 - read_point.x) + 4.0 + 14.0 * j as f32)
            / 50.0;
        let y = NP as f32
            * (-20.5 + (1.0 - 2.0 * (j % 2) as f32) * (4.5 - read_point.y) + 4.0 + 9.0 * i as f32)
            / 50.0;

        let point = pt2(x, y);

        shape[lion.line_index].push(point);
    }

    shape
}

fn main() {
    nannou::app(model).update(chapter_2::update).run();
}
