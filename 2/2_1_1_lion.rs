use common::{
    chapter_2::{self, Action, DesignShape, LION},
    Model, NoParams, Segment, Shape, Shapes, NP,
};
use nannou::prelude::*;

fn model(app: &App) -> Model<NoParams> {
    chapter_2::model(Box::new(calculate_shapes), app)
}

fn calculate_shapes(_params: &NoParams) -> Shapes {
    let mut shapes = Shapes::new();
    let mut shape = Shape::new();
    let mut segment = Segment::new();

    let mut lion = DesignShape::new(LION);

    while let Action::Continue(read_point, newsegment) = lion.calculate_point() {
        if newsegment {
            shape.push(segment);
            segment = Segment::new();
        }

        let x = NP as f32 * (read_point.x - 7.5) / 25.0;
        let y = NP as f32 * (read_point.y - 7.5) / 25.0;
        let point = pt2(x, y);

        segment.push(point);
    }

    shapes.push(shape);

    shapes
}

fn main() {
    nannou::app(model).update(chapter_2::update).run();
}
