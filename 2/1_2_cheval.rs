use common::{
    chapter_2::{self, Action, DesignShape, HORSE},
    Model, NoParams, Segment, Shape, Shapes, NP,
};
use nannou::prelude::*;

fn model(app: &App) -> Model<NoParams> {
    chapter_2::model(Box::new(calculate_shapes), app)
}

fn calculate_shapes(_params: &NoParams) -> Shapes {
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
    nannou::app(model).update(chapter_2::update).run();
}
