use common::{
    chapter_2::{self, Action, DesignShape, BIRD_FISH},
    Model, NoParams, Segment, Shape, Shapes, NP,
};
use nannou::prelude::*;

fn model(app: &App) -> Model<NoParams> {
    chapter_2::model(Box::new(calculate_shapes), app)
}

fn calculate_shapes(_params: &NoParams) -> Shapes {
    let mut shapes = Shapes::new();

    for i in 1..=4 {
        for j in 1..=4 {
            let mut shape = Shape::new();
            let mut segment = Segment::new();

            let mut bird_fish = DesignShape::new(BIRD_FISH);

            while let Action::Continue(read_point, newsegment) = bird_fish.calculate_point() {
                if newsegment {
                    shape.push(segment);
                    segment = Segment::new();
                }

                let x = NP as f32 * (read_point.y - 22.5 + 4.0 * i as f32 + 4.0 * j as f32) / 45.0;
                let y = NP as f32 * (read_point.x - 7.5 - 5.0 * i as f32 + 9.0 * j as f32) / 45.0;

                let point = pt2(x, y);

                segment.push(point);
            }

            shapes.push(shape);
        }
    }

    shapes
}

fn main() {
    nannou::app(model).update(chapter_2::update).run();
}
