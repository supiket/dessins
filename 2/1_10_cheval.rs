use common::{
    chapter_2::{self, Action, DesignShape, HORSE},
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

    for i in -4..=4 {
        for j in -4..=4 {
            let mut shape = Shape::new();
            let mut segment = Segment::new();

            let mut horse = DesignShape::new(HORSE);

            while let Action::Continue(read_point, newsegment) = horse.calculate_point() {
                if newsegment {
                    shape.push(segment);
                    segment = Segment::new();
                }

                let xx = (read_point.x + j as f32 * 20.0 - 20.0) / 100.0;
                let yy = (read_point.y + i as f32 * 20.0 - 20.0) / 100.0;
                let x = abs(xx).powf(0.7) * if xx < 0.0 { -1.0 } else { 1.0 } * NP as f32 / 2.0;
                let y = abs(yy).powf(0.7) * if yy < 0.0 { -1.0 } else { 1.0 } * NP as f32 / 2.0;

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
