use dessins_lib::{
    chapter_2::{self, Action, DesignShape, HORSE},
    Model, NoParamsInner, Segment, Shape, Shapes, NP,
};
use nannou::prelude::*;

fn model(app: &App) -> Model {
    chapter_2::model(app, Box::new(calculate_shapes))
}

fn update(_app: &App, model: &mut Model, update: Update) {
    dessins_lib::update(model, update);
}

fn calculate_shapes(_inner: &NoParamsInner) -> Shapes {
    let mut shapes = Shapes::new();

    for i in 0..=5 {
        for j in 0..pow(2, i as usize) {
            let mut shape = Shape::new();
            let mut segment = Segment::new();

            let mut horse = DesignShape::new(HORSE);

            let r = pow(0.5, i as usize);

            while let Action::Continue(read_point, newsegment) = horse.calculate_point() {
                if newsegment {
                    shape.push(segment);
                    segment = Segment::new();
                }

                let x = (j as f32 + read_point.x / 40.0) * NP as f32 * r - 0.5 * NP as f32;
                let y = (2.0 - 2.0 * r) * NP as f32 + read_point.y / 40.0 * NP as f32 * r
                    - 0.9 * NP as f32;
                let (x, y) = (x * 0.7, y * 0.7);
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
