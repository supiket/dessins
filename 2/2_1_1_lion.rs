use common::{
    chapter_2::{self, Action, DesignShape, Model, LION},
    Shape, Shapes, NP,
};
use nannou::prelude::*;

fn model(app: &App) -> Model {
    chapter_2::model(Box::new(calculate_shapes), app)
}

fn calculate_shapes() -> Shapes {
    vec![calculate_shape()]
}

fn calculate_shape() -> Shape {
    let mut shape = vec![vec![]];

    let mut lion = DesignShape::new(LION);

    while let Action::Continue(read_point, newline) = lion.calculate_point() {
        if newline {
            shape.push(vec![])
        }

        let x = NP as f32 * (read_point.x - 7.5) / 25.0;
        let y = NP as f32 * (read_point.y - 7.5) / 25.0;
        let point = pt2(x, y);

        shape[lion.line_index].push(point);
    }

    shape
}

fn main() {
    nannou::app(model).update(chapter_2::update).run();
}
