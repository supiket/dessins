use dessins_lib::{
    chapter_2::{self, Action, DesignShape, HORSE},
    update, Model, NoParamsInner, Segment, Shape, Shapes, NP,
};
use nannou::prelude::*;

fn model(app: &App) -> Model {
    chapter_2::model(app, Box::new(calculate_shapes))
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

                let x_ = (read_point.x + j as f32 * 20.0 - 20.0) * NP as f32 / 80.0;
                let y_ = (read_point.y + i as f32 * 20.0 - 20.0) * NP as f32 / 80.0;

                let di = (x_ * x_ + y_ * y_).sqrt();

                let an = if x_ != 0.0 {
                    (y_ / x_).atan() + PI * (1.0 - if x_ < 0.0 { -1.0 } else { 1.0 }) / 2.0
                } else {
                    PI / 2.0 * if y_ < 0.0 { -1.0 } else { 1.0 }
                };
                let di = di / NP as f32 * 3.0;
                let di = di / (1.0 + di) * NP as f32 * 0.65;

                let x = di * an.cos();
                let y = di * an.sin();

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
