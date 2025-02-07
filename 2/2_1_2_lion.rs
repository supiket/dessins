use common::{
    chapter_2::{self, Action, DesignShape, LION},
    Model, NoParamsInner, Segment, Shape, Shapes, NP,
};
use nannou::prelude::*;

fn model(app: &App) -> Model {
    chapter_2::model(app, Box::new(calculate_shapes))
}

fn update(_app: &App, model: &mut Model, update: Update) {
    common::update(model, update);
}

fn calculate_shapes(_inner: &NoParamsInner) -> Shapes {
    let mut shapes = Shapes::new();

    for i in 0..=4 {
        for j in 0..=2 {
            let mut shape = Shape::new();
            let mut segment = Segment::new();

            let mut lion = DesignShape::new(LION);

            while let Action::Continue(read_point, newsegment) = lion.calculate_point() {
                if newsegment {
                    shape.push(segment);
                    segment = Segment::new();
                }

                let x = NP as f32
                    * (-18.0
                        + (1.0 - 2.0 * (i % 2) as f32) * (7.0 - read_point.x)
                        + 4.0
                        + 14.0 * j as f32)
                    / 50.0;
                let y = NP as f32
                    * (-20.5
                        + (1.0 - 2.0 * (j % 2) as f32) * (4.5 - read_point.y)
                        + 4.0
                        + 9.0 * i as f32)
                    / 50.0;
                let point = pt2(x, y);

                segment.push(point);
            }

            shapes.push(shape);
        }
    }

    shapes
}

fn main() {
    nannou::app(model).update(update).run();
}
