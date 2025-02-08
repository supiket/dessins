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
        for j in 0..=1 {
            let mut shape = Shape::new();
            let mut segment = Segment::new();

            let mut horse = DesignShape::new(HORSE);

            let r = pow(0.5, i as usize);

            while let Action::Continue(read_point, newsegment) = horse.calculate_point() {
                if newsegment {
                    shape.push(segment);
                    segment = Segment::new();
                }

                let x = (1.0 - 2.0 * j as f32) * NP as f32 * read_point.x / 80.0 * r;
                let y = NP as f32 * (0.5 - r + read_point.y / 80.0 * r);
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
