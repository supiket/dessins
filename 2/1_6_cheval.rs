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
    nannou::app(model).update(chapter_2::update).run();
}
