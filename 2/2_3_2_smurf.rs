use common::{
    chapter_2::{self, Action, DesignShape, Model, SMURF},
    Shape, Shapes, NP,
};
use nannou::prelude::*;

fn model(app: &App) -> Model {
    chapter_2::model(Box::new(calculate_shapes), app)
}

fn calculate_shapes() -> Shapes {
    let mut shapes = vec![];

    for i in 0..7 {
        shapes.push(calculate_shape(i));
    }

    shapes
}

fn calculate_shape(i: i32) -> Shape {
    let mut shape = vec![vec![]];

    let mut smurf = DesignShape::new(SMURF);

    let k = (0.5).pow(i) as f32;

    while let Action::Continue(read_point, newline) = smurf.calculate_point() {
        if newline {
            shape.push(vec![])
        }

        let x = NP as f32 / 100.0 * read_point.x * k + 0.5 * NP as f32 - NP as f32 * k;
        let y = NP as f32 / 100.0 * read_point.y * k - 0.5 * NP as f32;

        let point = pt2(x, y);

        shape[smurf.line_index].push(point);
    }

    shape
}

fn main() {
    nannou::app(model).update(chapter_2::update).run();
}
