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

    while let Action::Continue(read_point, newline) = horse.calculate_point() {
        if newline {
            shape.push(vec![])
        }

        let x = co * read_point.x - si * read_point.y;
        let y = si * read_point.x + co * read_point.y;
        let (x, y) = (x * NP as f32 / 90.0, y * NP as f32 / 90.0);
        let point = pt2(x, y);

        shape[horse.line_index].push(point);
    }

    shape
}

fn main() {
    nannou::app(model).update(chapter_2::update).run();
}
