use super::{Action, DesignShape};
use crate::shapes::{sign, Segment, Shape, Shapes, NP};
use nannou::prelude::*;

#[derive(Clone, Debug, PartialEq, Reflect)]
pub enum ShapeProgram {
    Program1,
    Program2,
    Program3,
    Program4,
    Program5,
    Program6,
    Program7,
    Program8,
    Program9,
    Program10,
    Program11,
    Program12,
    Program13,
}

impl ShapeProgram {
    pub fn calculate_shapes(&mut self, design_shape: &mut DesignShape) -> Shapes {
        match self {
            Self::Program1 => self.program_1(design_shape),
            Self::Program2 => self.program_2(design_shape),
            Self::Program3 => self.program_3(design_shape),
            Self::Program4 => self.program_4(design_shape),
            Self::Program5 => self.program_5(design_shape),
            Self::Program6 => self.program_6(design_shape),
            Self::Program7 => self.program_7(design_shape),
            Self::Program8 => self.program_8(design_shape),
            Self::Program9 => self.program_9(design_shape),
            Self::Program10 => self.program_10(design_shape),
            Self::Program11 => self.program_11(design_shape),
            Self::Program12 => self.program_12(design_shape),
            Self::Program13 => self.program_13(design_shape),
        }
    }

    pub fn control(&mut self, ui: &mut egui::Ui) -> bool {
        let mut changed = false;

        ui.label("program");
        changed |= ui.radio_value(self, ShapeProgram::Program1, "o").changed();
        changed |= ui.radio_value(self, ShapeProgram::Program2, "oO").changed();
        changed |= ui
            .radio_value(self, ShapeProgram::Program3, "oOo")
            .changed();
        changed |= ui
            .radio_value(self, ShapeProgram::Program4, "oOoO")
            .changed();
        changed |= ui
            .radio_value(self, ShapeProgram::Program5, "oOoOo")
            .changed();
        changed |= ui
            .radio_value(self, ShapeProgram::Program6, "oOoOoO")
            .changed();
        changed |= ui
            .radio_value(self, ShapeProgram::Program7, "oOoOoOo")
            .changed();
        changed |= ui
            .radio_value(self, ShapeProgram::Program8, "oOoOoOoO")
            .changed();
        changed |= ui
            .radio_value(self, ShapeProgram::Program9, "oOoOoOoOo")
            .changed();
        changed |= ui
            .radio_value(self, ShapeProgram::Program10, "oOoOoOoOoO")
            .changed();
        changed |= ui
            .radio_value(self, ShapeProgram::Program11, "oOoOoOoOoOo")
            .changed();
        changed |= ui
            .radio_value(self, ShapeProgram::Program12, "oOoOoOoOoOoO")
            .changed();
        changed |= ui
            .radio_value(self, ShapeProgram::Program13, "oOoOoOoOoOoOo")
            .changed();

        changed
    }

    fn program_1(&mut self, design_shape: &mut DesignShape) -> Shapes {
        let mut shapes = Shapes::new();
        let mut shape = Shape::new();
        let mut segment = Segment::new();

        while let Action::Continue(read_point, newsegment) = design_shape.calculate_point() {
            if newsegment {
                shape.push(segment);
                segment = Segment::new();
            }

            let x = NP as f32 * (read_point.x - 20.0) / 40.0;
            let y = NP as f32 * (read_point.y - 20.0) / 40.0;
            let point = pt2(x, y);

            segment.push(point);
        }

        shapes.push(shape);

        shapes
    }

    fn program_2(&mut self, design_shape: &mut DesignShape) -> Shapes {
        let mut shapes = Shapes::new();

        for i in 0..=5 {
            let mut shape = Shape::new();
            let mut segment = Segment::new();

            design_shape.data_index = 0;

            let an = 2.0 * i as f32 * PI / 6.0 + PI / 12.0;
            let co = an.cos();
            let si = an.sin();

            while let Action::Continue(read_point, newsegment) = design_shape.calculate_point() {
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

    fn program_3(&mut self, design_shape: &mut DesignShape) -> Shapes {
        let mut shapes = Shapes::new();

        for i in 0..=5 {
            for j in 0..=1 {
                let mut shape = Shape::new();
                let mut segment = Segment::new();

                design_shape.data_index = 0;

                let r = pow(0.5, i as usize);

                while let Action::Continue(read_point, newsegment) = design_shape.calculate_point()
                {
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

    fn program_4(&mut self, design_shape: &mut DesignShape) -> Shapes {
        let mut shapes = Shapes::new();

        for i in 0..=15 {
            let mut shape = Shape::new();
            let mut segment = Segment::new();

            design_shape.data_index = 0;

            let an = 2.0 * i as f32 * PI / 6.0 + PI / 12.0;
            let co = an.cos();
            let si = an.sin();
            let r = pow(0.87, i as usize);

            while let Action::Continue(read_point, newsegment) = design_shape.calculate_point() {
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

    fn program_5(&mut self, design_shape: &mut DesignShape) -> Shapes {
        let mut shapes = Shapes::new();

        for i in 0..=5 {
            for j in 0..pow(2, i as usize) {
                let mut shape = Shape::new();
                let mut segment = Segment::new();

                design_shape.data_index = 0;

                let r = pow(0.5, i as usize);

                while let Action::Continue(read_point, newsegment) = design_shape.calculate_point()
                {
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

    fn program_6(&mut self, design_shape: &mut DesignShape) -> Shapes {
        let mut shapes = Shapes::new();

        for i in 0..=2 {
            for j in 0..=2 {
                let mut shape = Shape::new();
                let mut segment = Segment::new();

                design_shape.data_index = 0;

                while let Action::Continue(read_point, newsegment) = design_shape.calculate_point()
                {
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

    fn program_7(&mut self, design_shape: &mut DesignShape) -> Shapes {
        let mut shapes = Shapes::new();

        for i in -4..=4 {
            for j in -abs(i)..=abs(i) {
                let mut shape = Shape::new();
                let mut segment = Segment::new();

                design_shape.data_index = 0;

                while let Action::Continue(read_point, newsegment) = design_shape.calculate_point()
                {
                    if newsegment {
                        shape.push(segment);
                        segment = Segment::new();
                    }

                    let xx = (read_point.x + j as f32 * 20.0 - 20.0) / 100.0;
                    let yy = (read_point.y + i as f32 * 20.0 - 20.0) / 100.0;
                    let x = xx * 0.7 * NP as f32;
                    let y = yy * 0.7 * NP as f32;
                    let point = pt2(x, y);

                    segment.push(point);
                }

                shapes.push(shape);
            }
        }

        shapes
    }

    fn program_8(&mut self, design_shape: &mut DesignShape) -> Shapes {
        let mut shapes = Shapes::new();

        for i in -4..=4 {
            for j in -4..=4 {
                let mut shape = Shape::new();
                let mut segment = Segment::new();

                design_shape.data_index = 0;

                while let Action::Continue(read_point, newsegment) = design_shape.calculate_point()
                {
                    if newsegment {
                        shape.push(segment);
                        segment = Segment::new();
                    }

                    let xx = (read_point.x + j as f32 * 20.0 - 20.0) / 100.0;
                    let yy = (read_point.y + i as f32 * 20.0 - 20.0) / 100.0;
                    let x = xx * abs(xx) * 0.7 * NP as f32;
                    let y = yy * abs(yy) * 0.7 * NP as f32;
                    let point = pt2(x, y);

                    segment.push(point);
                }

                shapes.push(shape);
            }
        }

        shapes
    }

    fn program_9(&mut self, design_shape: &mut DesignShape) -> Shapes {
        let mut shapes = Shapes::new();

        for i in -4..=4 {
            for j in -4..=4 {
                let mut shape = Shape::new();
                let mut segment = Segment::new();

                design_shape.data_index = 0;

                while let Action::Continue(read_point, newsegment) = design_shape.calculate_point()
                {
                    if newsegment {
                        shape.push(segment);
                        segment = Segment::new();
                    }

                    let x_ = (read_point.x + j as f32 * 20.0 - 20.0) * NP as f32 / 80.0;
                    let y_ = (read_point.y + i as f32 * 20.0 - 20.0) * NP as f32 / 80.0;

                    let di = (x_ * x_ + y_ * y_).sqrt();

                    let an = if x_ != 0.0 {
                        (y_ / x_).atan() + PI * (1.0 - sign(x_)) / 2.0
                    } else {
                        PI / 2.0 * sign(y_)
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

    fn program_10(&mut self, design_shape: &mut DesignShape) -> Shapes {
        let mut shapes = Shapes::new();

        for i in -4..=4 {
            for j in -4..=4 {
                let mut shape = Shape::new();
                let mut segment = Segment::new();

                design_shape.data_index = 0;

                while let Action::Continue(read_point, newsegment) = design_shape.calculate_point()
                {
                    if newsegment {
                        shape.push(segment);
                        segment = Segment::new();
                    }

                    let xx = (read_point.x + j as f32 * 20.0 - 20.0) / 100.0;
                    let yy = (read_point.y + i as f32 * 20.0 - 20.0) / 100.0;
                    let x = abs(xx).powf(0.7) * sign(xx) * NP as f32 / 2.0;
                    let y = abs(yy).powf(0.7) * sign(yy) * NP as f32 / 2.0;

                    let point = pt2(x, y);

                    segment.push(point);
                }

                shapes.push(shape);
            }
        }

        shapes
    }

    fn program_11(&mut self, design_shape: &mut DesignShape) -> Shapes {
        let mut shapes = Shapes::new();

        for i in 0..=4 {
            for j in 0..=2 {
                let mut shape = Shape::new();
                let mut segment = Segment::new();

                design_shape.data_index = 0;

                while let Action::Continue(read_point, newsegment) = design_shape.calculate_point()
                {
                    if newsegment {
                        shape.push(segment);
                        segment = Segment::new();
                    }

                    let x = NP as f32
                        * (-18.0
                            + (1.0 - 2.0 * (i % 2) as f32) * (7.0 - read_point.x)
                            + 4.0
                            + 14.0 * j as f32)
                        / 50.0;
                    let y = NP as f32
                        * (-20.5
                            + (1.0 - 2.0 * (j % 2) as f32) * (4.5 - read_point.y)
                            + 4.0
                            + 9.0 * i as f32)
                        / 50.0;
                    let point = pt2(x, y);

                    segment.push(point);
                }

                shapes.push(shape);
            }
        }

        shapes
    }

    fn program_12(&mut self, design_shape: &mut DesignShape) -> Shapes {
        let mut shapes = Shapes::new();

        for i in 1..=4 {
            for j in 1..=4 {
                let mut shape = Shape::new();
                let mut segment = Segment::new();

                design_shape.data_index = 0;

                while let Action::Continue(read_point, newsegment) = design_shape.calculate_point()
                {
                    if newsegment {
                        shape.push(segment);
                        segment = Segment::new();
                    }

                    let x =
                        NP as f32 * (read_point.y - 22.5 + 4.0 * i as f32 + 4.0 * j as f32) / 45.0;
                    let y =
                        NP as f32 * (read_point.x - 7.5 - 5.0 * i as f32 + 9.0 * j as f32) / 45.0;

                    let point = pt2(x, y);

                    segment.push(point);
                }

                shapes.push(shape);
            }
        }

        shapes
    }

    fn program_13(&mut self, design_shape: &mut DesignShape) -> Shapes {
        let mut shapes = Shapes::new();

        for i in 0..=6 {
            let mut shape = Shape::new();
            let mut segment = Segment::new();

            design_shape.data_index = 0;

            let k = (0.5).pow(i) as f32;

            while let Action::Continue(read_point, newsegment) = design_shape.calculate_point() {
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
}
