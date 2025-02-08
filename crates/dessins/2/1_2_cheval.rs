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
        let mut shape = Shape::new();
        let mut segment = Segment::new();

        let mut horse = DesignShape::new(HORSE);

        let an = 2.0 * i as f32 * PI / 6.0 + PI / 12.0;
        let co = an.cos();
        let si = an.sin();

        while let Action::Continue(read_point, newsegment) = horse.calculate_point() {
            if newsegment {
                shape.push(segment);
                segment = Segment::new();
            }

            let x = co * read_point.x - si * read_point.y;
            let y = si * read_point.x + co * read_point.y;
            let (x, y) = (x * NP as f32 / 90.0, y * NP as f32 / 90.0);
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
