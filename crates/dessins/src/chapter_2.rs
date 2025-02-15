use crate::{Segment, Shape, Shapes, NP};
use nannou::prelude::*;
use ui_controlled_params::UiControlledParams;

pub const HORSE: &[f32] = &[
    1000.0, 10.0, 10.0, 8.0, 12.0, 9.0, 16.0, 12.0, 17.0, 13.0, 18.0, 14.0, 20.0, 1000.0, 13.0,
    18.0, 12.0, 19.0, 9.0, 21.0, 9.0, 20.0, 10.0, 19.0, 9.0, 17.0, 7.0, 20.0, 8.0, 22.0, 12.0,
    22.0, 1000.0, 12.0, 20.0, 12.0, 22.0, 13.0, 26.0, 16.0, 31.0, 18.0, 31.0, 19.0, 32.0, 1000.0,
    16.0, 31.0, 14.0, 31.0, 14.0, 32.0, 1000.0, 14.0, 31.0, 10.0, 30.0, 12.0, 31.0, 10.0, 32.0,
    10.0, 34.0, 11.0, 34.0, 11.0, 33.0, 10.0, 33.0, 1000.0, 12.0, 32.0, 13.0, 31.0, 1000.0, 10.0,
    34.0, 16.0, 36.0, 1000.0, 16.0, 35.0, 16.0, 37.0, 18.0, 35.0, 17.0, 34.0, 1000.0, 17.0, 36.0,
    20.0, 36.0, 22.0, 32.0, 19.0, 26.0, 1000.0, 20.0, 36.0, 22.0, 36.0, 22.0, 34.0, 24.0, 32.0,
    24.0, 30.0, 19.0, 26.0, 18.0, 23.0, 21.0, 22.0, 21.0, 24.0, 30.0, 30.0, 34.0, 31.0, 36.0, 31.0,
    33.0, 26.0, 32.0, 22.0, 28.0, 22.0, 27.0, 20.0, 29.0, 17.0, 30.0, 19.0, 29.0, 20.0, 29.0, 21.0,
    32.0, 19.0, 33.0, 18.0, 32.0, 17.0, 29.0, 16.0, 28.0, 12.0, 30.0, 10.0, 21.0, 4.0, 21.0, 2.0,
    18.0, 3.0, 19.0, 6.0, 24.0, 10.0, 24.0, 12.0, 22.0, 14.0, 22.0, 16.0, 23.0, 17.0, 1000.0, 22.0,
    16.0, 17.0, 16.0, 16.0, 17.0, 17.0, 18.0, 1000.0, 16.0, 17.0, 16.0, 16.0, 10.0, 14.0, 10.0,
    12.0, 12.0, 11.0, 10.0, 10.0, 1000.0, 21.0, 21.0, 22.0, 24.0, 30.0, 30.0, 1000.0, 24.0, 24.0,
    34.0, 28.0, 1000.0, 25.0, 23.0, 33.0, 26.0, 1000.0, 25.0, 21.0, 27.0, 20.0, 1000.0, 23.0, 21.0,
    24.0, 19.0, 1000.0, 27.0, 20.0, 22.0, 19.0, 22.0, 21.0, 1000.0, 22.0, 19.0, 21.0, 20.0, 1000.0,
    13.0, 34.0, 15.0, 35.0, 16.0, 34.0, 16.0, 33.0, 1000.0, 15.0, 35.0, 15.0, 34.0, 16.0, 34.0,
    15.0, 34.0, 15.0, 35.0, 1000.0, 24.0, 12.0, 26.0, 10.0, 19.0, 5.0, 19.0, 3.0, 1000.0, 28.0,
    22.0, 25.0, 22.0, 2000.0,
];

pub const LION: &[f32] = &[
    1000.0, -2.5, 0.0, -2.0, 1.0, -1.0, 2.0, 0.0, 7.0, 1.0, 7.0, 2.0, 8.0, 2.0, 11.0, 3.0, 14.0,
    3.5, 13.5, 2.5, 11.0, 2.5, 9.0, 1000.0, 3.5, 13.5, 4.0, 13.0, 3.0, 11.0, 3.0, 9.0, 3.0, 11.0,
    4.0, 13.0, 5.0, 12.0, 3.5, 11.0, 3.5, 9.0, 3.5, 11.0, 5.0, 12.0, 5.0, 11.0, 4.0, 10.0, 4.0,
    9.0, 8.0, 9.0, 7.0, 11.0, 8.0, 13.0, 10.0, 14.0, 12.0, 13.0, 13.0, 11.0, 12.0, 11.0, 11.0,
    10.0, 12.0, 8.0, 13.0, 7.0, 14.0, 2.0, 15.0, 2.0, 16.0, 1.0, 16.0, 0.0, 12.0, 0.0, 12.0, 2.0,
    11.0, 5.0, 11.5, 6.0, 11.0, 5.0, 9.0, 3.0, 9.0, 2.0, 10.0, 1.0, 10.0, 0.0, 6.0, 0.0, 7.0, 2.0,
    8.0, 6.0, 7.0, 2.0, 6.0, 4.0, 4.0, 5.0, 5.0, 7.0, 4.0, 8.0, 5.0, 7.0, 4.0, 5.0, 2.0, 4.0, 1.0,
    2.0, 2.0, 2.0, 3.0, 1.0, 2.5, 0.0, -2.5, 0.0, 1000.0, 6.0, 4.0, 7.5, 3.5, 1000.0, 12.0, 11.0,
    10.0, 10.5, 9.0, 10.5, 1000.0, 12.5, 12.0, 12.0, 12.0, 11.0, 11.5, 12.0, 12.0, 12.0, 12.5,
    11.5, 12.5, 10.5, 13.0, 10.0, 13.0, 10.0, 13.5, 10.5, 13.5, 10.5, 13.0, 11.5, 12.5, 12.0, 12.5,
    12.0, 13.0, 1000.0, 7.5, 12.0, 8.5, 12.0, 8.5, 11.5, 2000.0,
];

pub const BIRD_FISH: &[f32] = &[
    1000.0, 0.0, 0.0, 2.0, 0.0, 4.0, 1.0, 4.0, 2.0, 3.0, 2.0, 2.0, 3.0, 4.0, 5.0, 4.0, 6.0, 2.0,
    5.0, 2.0, 6.0, -1.0, 5.0, -2.0, 3.0, -1.0, 2.0, -2.0, 2.0, -3.0, 3.0, -4.0, 3.0, -5.0, 2.0,
    -4.0, 2.0, 0.0, 0.0, 1000.0, -5.0, 2.0, -5.0, 1.0, -7.0, -1.0, -6.0, -2.0, -5.0, -2.0, -5.0,
    -3.0, -2.0, -2.0, -2.0, -3.0, 0.0, -2.0, 1.0, -1.0, 2.0, -1.0, 3.0, -2.0, 4.0, -2.0, 3.0, -1.0,
    4.0, 1.0, 1000.0, 2.0, 5.0, 0.0, 4.0, 0.0, 2.0, 1000.0, -2.0, 1.0, -5.0, 1.0, -4.0, -1.0, -3.0,
    0.0, -3.0, -1.0, -4.0, -1.0, -5.0, -2.0, 0.0, -2.0, 1000.0, -7.0, -1.0, -6.0, -1.0, 1000.0,
    -4.0, 2.5, -4.0, 2.8, -4.3, 2.8, -4.3, 2.5, -4.0, 2.5, 1000.0, -5.0, 0.0, -5.5, 0.0, -5.5, 0.5,
    -5.0, 0.5, -5.0, 0.0, 2000.0,
];

pub const SMURF: &[f32] = &[
    1000.0, 12.0, 12.0, 14.0, 8.0, 14.0, 4.0, 12.0, 2.0, 8.0, 2.0, 4.0, 4.0, 0.0, 10.0, 0.0, 20.0,
    4.0, 26.0, 6.0, 28.0, 12.0, 28.0, 14.0, 26.0, 14.0, 22.0, 12.0, 16.0, 12.0, 12.0, 20.0, 14.0,
    24.0, 14.0, 28.0, 12.0, 28.0, 10.0, 26.0, 4.0, 28.0, 0.0, 36.0, 0.0, 38.0, 2.0, 40.0, 10.0,
    40.0, 22.0, 36.0, 26.0, 28.0, 26.0, 26.0, 22.0, 28.0, 14.0, 28.0, 12.0, 28.0, 14.0, 27.0, 18.0,
    18.0, 18.0, 16.0, 20.0, 16.0, 18.0, 20.0, 14.0, 16.0, 18.0, 12.0, 16.0, 1000.0, 16.0, 20.0,
    16.0, 24.0, 20.0, 32.0, 20.0, 34.0, 20.0, 32.0, 12.0, 34.0, 12.0, 32.0, 10.0, 28.0, 1000.0,
    4.0, 26.0, 2.0, 28.0, 4.0, 30.0, 8.0, 30.0, 6.0, 32.0, 6.0, 34.0, 6.0, 32.0, 4.0, 32.0, 2.0,
    30.0, 2.0, 28.0, 1000.0, 8.0, 30.0, 8.0, 36.0, 10.0, 38.0, 1000.0, 4.0, 32.0, 4.0, 34.0, 8.0,
    38.0, 6.0, 40.0, 6.0, 42.0, 8.0, 44.0, 10.0, 44.0, 10.0, 42.0, 12.0, 42.0, 12.0, 38.0, 16.0,
    36.0, 32.0, 36.0, 38.0, 40.0, 40.0, 44.0, 38.0, 42.0, 36.0, 46.0, 30.0, 48.0, 36.0, 48.0, 40.0,
    44.0, 40.0, 56.0, 36.0, 62.0, 32.0, 64.0, 24.0, 64.0, 18.0, 62.0, 16.0, 60.0, 16.0, 58.0, 18.0,
    56.0, 24.0, 56.0, 22.0, 56.0, 20.0, 53.0, 28.0, 56.0, 22.0, 54.0, 28.0, 54.0, 32.0, 52.0, 34.0,
    48.0, 32.0, 52.0, 28.0, 48.0, 30.0, 46.0, 28.0, 44.0, 1000.0, 28.0, 48.0, 22.0, 48.0, 24.0,
    48.0, 24.0, 52.0, 22.0, 54.0, 18.0, 52.0, 18.0, 50.0, 20.0, 48.0, 12.0, 48.0, 16.0, 48.0, 18.0,
    50.0, 16.0, 48.0, 16.0, 50.0, 18.0, 52.0, 16.0, 50.0, 16.0, 48.0, 14.0, 46.0, 16.0, 44.0,
    1000.0, 12.0, 48.0, 10.0, 44.0, 1000.0, 16.0, 46.0, 18.0, 44.0, 1000.0, 18.0, 46.0, 26.0, 46.0,
    24.0, 46.0, 24.0, 44.0, 22.0, 42.0, 20.0, 44.0, 20.0, 46.0, 1000.0, 22.0, 42.0, 22.0, 44.0,
    24.0, 44.0, 1000.0, 28.0, 46.0, 26.0, 44.0, 1000.0, 24.0, 54.0, 25.0, 52.0, 1000.0, 27.0, 52.0,
    28.0, 54.0, 30.0, 52.0, 1000.0, 25.0, 49.0, 26.0, 50.0, 27.0, 49.0, 1000.0, 36.0, 38.0, 40.0,
    38.0, 42.0, 40.0, 48.0, 40.0, 48.0, 42.0, 50.0, 42.0, 52.0, 40.0, 50.0, 36.0, 48.0, 36.0, 48.0,
    38.0, 48.0, 38.0, 48.0, 36.0, 46.0, 34.0, 48.0, 36.0, 48.0, 26.0, 46.0, 24.0, 46.0, 32.0, 46.0,
    30.0, 42.0, 30.0, 44.0, 28.0, 44.0, 26.0, 42.0, 24.0, 40.0, 26.0, 40.0, 32.0, 42.0, 32.0, 28.0,
    32.0, 30.0, 32.0, 32.0, 26.0, 1000.0, 44.0, 26.0, 44.0, 24.0, 46.0, 24.0, 1000.0, 42.0, 38.0,
    44.0, 36.0, 44.0, 32.0, 2000.0,
];

pub struct DesignShape {
    pub data: Vec<f32>,
    pub data_index: usize,
}

#[derive(UiControlledParams)]
#[params(Shape)]
pub struct ParamsInner {
    #[param]
    pub raw_shape: RawShape,
    #[param]
    pub shape_program: ShapeProgram,
}

pub enum Action {
    Continue(Point2, bool),
    Break,
}

#[derive(PartialEq)]
pub enum RawShape {
    Horse,
    Lion,
    BirdFish,
    Smurf,
}

#[derive(PartialEq)]
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

fn sign(val: f32) -> f32 {
    if val < 0.0 {
        -1.0
    } else if val == 0.0 {
        val
    } else {
        1.0
    }
}

impl RawShape {
    fn ui_elements(&mut self, ui: &mut egui::Ui) -> bool {
        let mut changed = false;

        ui.label("shape");
        changed |= ui.radio_value(self, RawShape::Horse, "horse").changed();
        changed |= ui.radio_value(self, RawShape::Lion, "lion").changed();
        changed |= ui
            .radio_value(self, RawShape::BirdFish, "bird-fish")
            .changed();
        changed |= ui.radio_value(self, RawShape::Smurf, "smurf").changed();

        changed
    }
}

impl ShapeProgram {
    fn ui_elements(&mut self, ui: &mut egui::Ui) -> bool {
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
}

impl DesignShape {
    pub fn new(data: &[f32]) -> Self {
        Self {
            data: data.to_vec(),
            data_index: 0,
        }
    }

    pub fn calculate_point(&mut self) -> Action {
        let mut new_segment = false;

        let mut a = self.data[self.data_index];
        self.data_index += 1;
        if a == 2000.0 {
            return Action::Break;
        }
        if a == 1000.0 {
            a = self.data[self.data_index];
            self.data_index += 1;
            new_segment = true;
        }
        let b = self.data[self.data_index];
        let point = pt2(a, b);
        self.data_index += 1;

        Action::Continue(point, new_segment)
    }
}

impl ParamsInner {
    pub fn calculate_shapes(&mut self) -> Shapes {
        let raw_shape = match self.raw_shape {
            RawShape::Horse => HORSE,
            RawShape::Lion => LION,
            RawShape::BirdFish => BIRD_FISH,
            RawShape::Smurf => SMURF,
        };
        let mut design_shape = DesignShape::new(raw_shape);

        match self.shape_program {
            ShapeProgram::Program1 => self.program_1(&mut design_shape),
            ShapeProgram::Program2 => self.program_2(&mut design_shape),
            ShapeProgram::Program3 => self.program_3(&mut design_shape),
            ShapeProgram::Program4 => self.program_4(&mut design_shape),
            ShapeProgram::Program5 => self.program_5(&mut design_shape),
            ShapeProgram::Program6 => self.program_6(&mut design_shape),
            ShapeProgram::Program7 => self.program_7(&mut design_shape),
            ShapeProgram::Program8 => self.program_8(&mut design_shape),
            ShapeProgram::Program9 => self.program_9(&mut design_shape),
            ShapeProgram::Program10 => self.program_10(&mut design_shape),
            ShapeProgram::Program11 => self.program_11(&mut design_shape),
            ShapeProgram::Program12 => self.program_12(&mut design_shape),
            ShapeProgram::Program13 => self.program_13(&mut design_shape),
        }
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

impl Default for Params {
    fn default() -> Self {
        let inner = ParamsInner {
            raw_shape: RawShape::Horse,
            shape_program: ShapeProgram::Program1,
        };

        Self {
            inner,
            calculate_shapes: Box::new(ParamsInner::calculate_shapes),
            ui_elements: Box::new(ParamsInner::ui_elements),
        }
    }
}
