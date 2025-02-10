use dessins_lib::{
    chapter_2::{Action, DesignShape, HORSE},
    no_params::NoParamsInner,
    update, Model, Segment, Shape, Shapes, NP,
};
use nannou::prelude::*;

fn model(app: &App) -> Model {
    NoParamsInner().model(app, Box::new(calculate_shapes))
}

fn calculate_shapes(_inner: &mut NoParamsInner) -> Shapes {
    let mut shapes = Shapes::new();

    for i in 0..=2 {
        for j in 0..=2 {
            let mut shape = Shape::new();
            let mut segment = Segment::new();

            let mut horse = DesignShape::new(HORSE);

            while let Action::Continue(read_point, newsegment) = horse.calculate_point() {
                if newsegment {
                    shape.push(segment);
                    segment = Segment::new();
                }

                let x = NP as f32 * ((read_point.x + j as f32 * 20.0) / 80.0 - 0.5);
                let y = NP as f32 * ((read_point.y + i as f32 * 20.0) / 80.0 - 0.5);
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
