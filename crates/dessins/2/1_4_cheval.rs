use dessins_lib::{
    chapter_2::{Action, DesignShape, HORSE},
    no_params::NoParamsInner,
    update, Model, Segment, Shape, Shapes, NP,
};
use nannou::prelude::*;

fn model(app: &App) -> Model {
    NoParamsInner().model(app, Box::new(calculate_shapes))
}

fn calculate_shapes(_inner: &NoParamsInner) -> Shapes {
    let mut shapes = Shapes::new();

    for i in 0..=15 {
        let mut shape = Shape::new();
        let mut segment = Segment::new();

        let mut horse = DesignShape::new(HORSE);

        let an = 2.0 * i as f32 * PI / 6.0 + PI / 12.0;
        let co = an.cos();
        let si = an.sin();
        let r = pow(0.87, i as usize);

        while let Action::Continue(read_point, newsegment) = horse.calculate_point() {
            if newsegment {
                shape.push(segment);
                segment = Segment::new();
            }

            let x_ = 0.15 + read_point.x / 110.0;
            let y_ = 0.15 + read_point.y / 110.0;
            let x = NP as f32 * (r * (co * x_ - si * y_));
            let y = NP as f32 * (r * (si * x_ + co * y_));
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
