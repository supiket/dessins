use common::{
    chapter_2::{self, Action, DesignShape, Model, HORSE},
    Line, Shape, Shapes, NP,
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

    let mut horse = DesignShape::new(HORSE);

    while let Action::Continue(read_point, newline) = horse.calculate_point() {
        if newline {
            shape.push(Line::default())
        }

        let x = NP as f32 * (read_point.x - 20.0) / 40.0;
        let y = NP as f32 * (read_point.y - 20.0) / 40.0;
        let point = pt2(x, y);

        shape[horse.line_index].push(point);
    }

    shape
}

fn main() {
    nannou::app(model).update(chapter_2::update).run();
}
