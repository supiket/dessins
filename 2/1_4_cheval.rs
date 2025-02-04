use common::{
    chapter_2::{self, Action, DesignShape, Model, HORSE},
    Shape, Shapes, NP,
};
use nannou::prelude::*;

const N: i32 = 16;

fn model(app: &App) -> Model {
    chapter_2::model(Box::new(calculate_shapes), app)
}

fn calculate_shapes() -> Shapes {
    let mut shapes = vec![];

    for i in 0..N {
        shapes.push(calculate_shape(i));
    }

    shapes
}

fn calculate_shape(i: i32) -> Shape {
    let mut shape = vec![vec![]];

    let mut horse = DesignShape::new(HORSE);

    let an = 2.0 * i as f32 * PI / 6.0 + PI / 12.0;
    let co = an.cos();
    let si = an.sin();
    let r = pow(0.87, i as usize);

    while let Action::Continue(read_point, newline) = horse.calculate_point() {
        if newline {
            shape.push(vec![])
        }

        let x_ = 0.15 + read_point.x / 110.0;
        let y_ = 0.15 + read_point.y / 110.0;
        let x = NP as f32 * (r * (co * x_ - si * y_));
        let y = NP as f32 * (r * (si * x_ + co * y_));
        let point = pt2(x, y);

        shape[horse.line_index].push(point);
    }

    shape
}

fn main() {
    nannou::app(model).update(chapter_2::update).run();
}
