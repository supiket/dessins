use common::{
    chapter_2::{self, Action, DesignShape, SMURF},
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

    for i in 0..=6 {
        let mut shape = Shape::new();
        let mut segment = Segment::new();

        let mut smurf = DesignShape::new(SMURF);

        let k = (0.5).pow(i) as f32;

        while let Action::Continue(read_point, newsegment) = smurf.calculate_point() {
            if newsegment {
                shape.push(segment);
                segment = Segment::new();
            }

            let x = NP as f32 / 100.0 * read_point.x * k + 0.5 * NP as f32 - NP as f32 * k;
            let y = NP as f32 / 100.0 * read_point.y * k - 0.5 * NP as f32;
            let point = pt2(x, y);

            segment.push(point);
        }

        shapes.push(shape);
    }

    shapes
}

fn main() {
    nannou::app(model).update(update).run();
}
