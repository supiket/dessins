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

    for i in -N..=N {
        for j in -N..=N {
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

        let x_ = (read_point.x + j as f32 * 20.0 - 20.0) * NP as f32 / 80.0;
        let y_ = (read_point.y + i as f32 * 20.0 - 20.0) * NP as f32 / 80.0;

        let di = (x_ * x_ + y_ * y_).sqrt();

        let an = if x_ != 0.0 {
            (y_ / x_).atan() + PI * (1.0 - if x_ < 0.0 { -1.0 } else { 1.0 }) / 2.0
        } else {
            PI / 2.0 * if y_ < 0.0 { -1.0 } else { 1.0 }
        };
        let di = di / NP as f32 * 3.0;
        let di = di / (1.0 + di) * NP as f32 * 0.65;

        let x = di * an.cos();
        let y = di * an.sin();

        let point = pt2(x, y);

        shape[horse.line_index].push(point);
    }

    shape
}

fn main() {
    nannou::app(model).update(chapter_2::update).run();
}
