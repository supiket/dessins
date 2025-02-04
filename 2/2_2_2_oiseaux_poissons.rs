use common::{
    chapter_2::{self, Action, DesignShape, Model, BIRD_FISH},
    Shape, Shapes, NP,
};
use nannou::prelude::*;

fn model(app: &App) -> Model {
    chapter_2::model(Box::new(calculate_shapes), app)
}

fn calculate_shapes() -> Shapes {
    let mut shapes = vec![];

    for i in 1..=4 {
        for j in 1..=4 {
            shapes.push(calculate_shape(i, j));
        }
    }

    shapes
}

fn calculate_shape(i: i32, j: i32) -> Shape {
    let mut shape = vec![vec![]];

    let mut bird_fish = DesignShape::new(BIRD_FISH);

    while let Action::Continue(read_point, newline) = bird_fish.calculate_point() {
        if newline {
            shape.push(vec![])
        }

        let x = NP as f32 * (read_point.y - 22.5 + 4.0 * i as f32 + 4.0 * j as f32) / 45.0;
        let y = NP as f32 * (read_point.x - 7.5 - 5.0 * i as f32 + 9.0 * j as f32) / 45.0;

        let point = pt2(x, y);

        shape[bird_fish.line_index].push(point);
    }

    shape
}

fn main() {
    nannou::app(model).update(chapter_2::update).run();
}
