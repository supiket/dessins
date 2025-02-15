use crate::{chapter_1::polygon, Segment, Shape, Shapes, NP};
use nannou::prelude::*;
use ui_controlled_params::UiControlledParams;

pub type OuterSegment = Segment;
pub type InnerSegment = Segment;

#[derive(UiControlledParams)]
#[params(SimpleDeformedFractal)]
pub struct ParamsInner {
    #[param]
    pub deformation: Deformation,
    #[param(range(3..=10))]
    pub m: usize, // # of segments in starting curve
    #[param(range(1..=10))]
    pub n: usize, // # of sub-segments per segment
    #[param(range(1..=10))]
    pub k: usize, // depth
    pub positions: Vec<Point2>,
    #[param(range(0.0..=30.0))]
    pub lengths: Vec<f32>,
    #[param(pi)]
    pub angles: Vec<f32>,
}

#[derive(PartialEq)]
pub enum Deformation {
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

impl Deformation {
    fn ui_elements(&mut self, ui: &mut egui::Ui) -> bool {
        let mut changed = false;

        ui.label("deformation");
        changed |= ui.radio_value(self, Deformation::Program1, "o").changed();
        changed |= ui.radio_value(self, Deformation::Program2, "oO").changed();
        changed |= ui.radio_value(self, Deformation::Program3, "oOo").changed();
        changed |= ui
            .radio_value(self, Deformation::Program4, "oOoO")
            .changed();
        changed |= ui
            .radio_value(self, Deformation::Program5, "oOoOo")
            .changed();
        changed |= ui
            .radio_value(self, Deformation::Program6, "oOoOoO")
            .changed();
        changed |= ui
            .radio_value(self, Deformation::Program7, "oOoOoOo")
            .changed();
        changed |= ui
            .radio_value(self, Deformation::Program8, "oOoOoOo")
            .changed();
        changed |= ui
            .radio_value(self, Deformation::Program9, "oOoOoOoO")
            .changed();
        changed |= ui
            .radio_value(self, Deformation::Program10, "oOoOoOoOo")
            .changed();
        changed |= ui
            .radio_value(self, Deformation::Program11, "oOoOoOoOoO")
            .changed();
        changed |= ui
            .radio_value(self, Deformation::Program12, "oOoOoOoOoOo")
            .changed();

        changed
    }
}

impl ParamsInner {
    pub fn calculate_shapes(&mut self) -> Shapes {
        let deforme_point = match self.deformation {
            Deformation::Program1 => Self::deforme_point_1,
            Deformation::Program2 => Self::deforme_point_2,
            Deformation::Program3 => Self::deforme_point_3,
            Deformation::Program4 => Self::deforme_point_4,
            Deformation::Program5 => Self::deforme_point_5,
            Deformation::Program6 => Self::deforme_point_6,
            Deformation::Program7 => Self::deforme_point_7,
            Deformation::Program8 => Self::deforme_point_8,
            Deformation::Program9 => Self::deforme_point_9,
            Deformation::Program10 => Self::deforme_point_10,
            Deformation::Program11 => Self::deforme_point_11,
            Deformation::Program12 => Self::deforme_point_12,
        };

        match self.deformation {
            Deformation::Program9 => self.set_initials_2(),
            Deformation::Program10 => self.set_initials_2(),
            Deformation::Program11 => self.set_initials_2(),
            Deformation::Program12 => self.set_initials_2(),
            _ => self.set_initials(),
        }

        let mut shapes = Shapes::new();
        let mut shape = Shape::new();

        for ii in 0..self.m {
            let mut segment = Segment::new();

            let source = self.positions[ii];
            let destination = self.positions[ii + 1];
            let diff = destination - source;

            let mut point = source;
            segment.push(deforme_point(point));

            let angle = if diff.x == 0.0 {
                PI / 2.0 * sign(diff.y)
            } else {
                (diff.y / diff.x).atan()
            } + if diff.x < 0.0 { PI } else { 0.0 };

            let length = diff.length();

            for i in 0..(self.n).pow(self.k as u32) {
                let mut current_length = length;
                let mut current_angle = angle;
                let mut t1 = i;
                if self.k != 0 {
                    for j in (0..self.k).rev() {
                        let r = (self.n).pow(j as u32);
                        let t2 = t1 / r;
                        current_angle += self.angles[t2];
                        current_length *= self.lengths[t2];
                        t1 -= t2 * r;
                    }
                }
                point += pt2(
                    current_length * current_angle.cos(),
                    current_length * current_angle.sin(),
                );
                segment.push(deforme_point(point));
            }
            shape.push(segment);
        }

        shapes.push(shape);

        shapes
    }

    fn deforme_point_1(point: Point2) -> Point2 {
        let diff = pt2(point.x / NP as f32 * 2.0, point.y / NP as f32 * 2.0);
        let mut dh = diff.length();

        let angle = if diff.x == 0.0 {
            PI / 2.0 * sign(diff.y)
        } else {
            (diff.y / diff.x).atan() - PI * (sign(diff.x) - 1.0) / 2.0
        };
        dh = dh * dh;

        let x = dh * angle.cos() * NP as f32 / 2.0;
        let y = dh * angle.sin() * NP as f32 / 2.0;

        pt2(x, y)
    }

    fn deforme_point_2(point: Point2) -> Point2 {
        let diff = pt2(point.x / NP as f32 * 2.0, point.y / NP as f32 * 2.0);
        let mut dh = diff.length();
        let mut angle = if diff.x == 0.0 {
            PI / 2.0 * sign(diff.y)
        } else {
            (diff.y / diff.x).atan() - PI * (sign(diff.x) - 1.0) / 2.0
        };

        angle += PI * dh;
        dh = dh.powf(4.0);

        let x = dh * angle.cos() * NP as f32 / 2.0;
        let y = dh * angle.sin() * NP as f32 / 2.0;

        pt2(x, y)
    }

    fn deforme_point_3(point: Point2) -> Point2 {
        let diff = pt2(point.x / NP as f32 * 2.0, point.y / NP as f32 * 2.0);
        let mut dh = diff.length();
        let mut angle = if diff.x == 0.0 {
            PI / 2.0 * sign(diff.y)
        } else {
            (diff.y / diff.x).atan() - PI * (sign(diff.x) - 1.0) / 2.0
        };

        angle += PI / 4.0 * (2.0 * PI * dh).sin();
        dh = dh.powf(5.0);

        let x = dh * angle.cos() * NP as f32 / 2.0;
        let y = dh * angle.sin() * NP as f32 / 2.0;

        pt2(x, y)
    }

    fn deforme_point_4(point: Point2) -> Point2 {
        let diff = pt2(point.x / NP as f32 * 2.0, point.y / NP as f32 * 2.0);
        let mut dh = diff.length();
        let mut angle = if diff.x == 0.0 {
            PI / 2.0 * sign(diff.y)
        } else {
            (diff.y / diff.x).atan() - PI * (sign(diff.x) - 1.0) / 2.0 * sign(diff.y)
        };

        angle = angle.powf(3.0) / PI.powf(2.0);
        dh = dh.powf(6.0);

        let x = dh * angle.cos() * NP as f32 / 2.0;
        let y = dh * angle.sin() * NP as f32 / 2.0;

        pt2(x, y)
    }

    fn deforme_point_5(point: Point2) -> Point2 {
        let diff = pt2(point.x / NP as f32 * 2.0, point.y / NP as f32 * 2.0);
        let mut dh = diff.length();
        let mut angle = if diff.x == 0.0 {
            PI / 2.0 * sign(diff.y)
        } else {
            (diff.y / diff.x).atan() - PI * (sign(diff.x) - 1.0) / 2.0 * sign(diff.y)
        };

        angle = 4.0 * angle.powf(3.0) / PI.powf(2.0);
        dh = dh.powf(6.0);

        let x = dh * angle.cos() * NP as f32 / 2.0;
        let y = dh * angle.sin() * NP as f32 / 2.0;

        pt2(x, y)
    }

    fn deforme_point_6(point: Point2) -> Point2 {
        let diff = pt2(point.x / NP as f32 * 2.0, point.y / NP as f32 * 2.0);
        let mut dh = diff.length();
        let mut angle = if diff.x == 0.0 {
            PI / 2.0 * sign(diff.y)
        } else {
            (diff.y / diff.x).atan() - PI * (sign(diff.x) - 1.0) / 2.0 * sign(diff.y)
        };

        angle *= 10.0;
        dh = dh.powf(5.0);

        let x = dh * angle.cos() * NP as f32 / 2.0;
        let y = dh * angle.sin() * NP as f32 / 2.0;

        pt2(x, y)
    }

    fn deforme_point_7(point: Point2) -> Point2 {
        let diff = pt2(point.x / NP as f32 * 2.0, point.y / NP as f32 * 2.0);
        let mut dh = diff.length();
        let mut angle = if diff.x == 0.0 {
            PI / 2.0 * sign(diff.y)
        } else {
            (diff.y / diff.x).atan() - PI * (sign(diff.x) - 1.0) / 2.0 * sign(diff.y)
        };

        angle += PI / 18.0 * (6.0 * PI * dh).sin();
        dh = dh.powf(5.0);

        let x = dh * angle.cos() * NP as f32 / 2.0;
        let y = dh * angle.sin() * NP as f32 / 2.0;

        pt2(x, y)
    }

    fn deforme_point_8(point: Point2) -> Point2 {
        let diff = pt2(point.x / NP as f32 * 2.0, point.y / NP as f32 * 2.0);
        let mut dh = diff.length();
        let mut angle = if diff.x == 0.0 {
            PI / 2.0 * sign(diff.y)
        } else {
            (diff.y / diff.x).atan() - PI * (sign(diff.x) - 1.0) / 2.0 * sign(diff.y)
        };

        angle *= 20.0;
        dh = dh.powf(5.0);

        let x = dh * angle.cos() * NP as f32 / 2.0;
        let y = dh * angle.sin() * NP as f32 / 2.0;

        pt2(x, y)
    }

    fn deforme_point_9(point: Point2) -> Point2 {
        let diff = pt2(point.x / NP as f32 * 2.0, point.y / NP as f32 * 2.0);
        let dh = diff.length();

        let mut angle = if diff.x == 0.0 {
            PI / 2.0 * sign(diff.y)
        } else {
            (diff.y / diff.x).atan() - PI * (sign(diff.x) - 1.0) / 2.0 * sign(diff.y)
        };

        angle *= angle / PI * sign(angle);

        let x = dh * angle.cos() * NP as f32 / 2.0;
        let y = 2.0 * dh * angle.sin() * NP as f32 / 2.0;

        pt2(x, y)
    }

    fn deforme_point_10(point: Point2) -> Point2 {
        let diff = pt2(point.x / NP as f32 * 2.0, point.y / NP as f32 * 2.0);
        let dh = diff.length();

        let mut angle = if diff.x == 0.0 {
            PI / 2.0 * sign(diff.y)
        } else {
            (diff.y / diff.x).atan() - PI * (sign(diff.x) - 1.0) / 2.0 * sign(diff.y)
        };

        angle += PI / 2.0 * (1.0 - dh);

        let x = dh * angle.cos() * NP as f32 / 2.0;
        let y = 2.0 * dh * angle.sin() * NP as f32 / 2.0;

        pt2(x, y)
    }

    fn deforme_point_11(point: Point2) -> Point2 {
        let diff = pt2(point.x / NP as f32 * 2.0, point.y / NP as f32 * 2.0);
        let mut dh = diff.length();

        let angle = if diff.x == 0.0 {
            PI / 2.0 * sign(diff.y)
        } else {
            (diff.y / diff.x).atan() - PI * (sign(diff.x) - 1.0) / 2.0 * sign(diff.y)
        };

        dh = dh.powf(2.0);

        let x = dh * angle.cos() * NP as f32 / 2.0;
        let y = 2.0 * dh * angle.sin() * NP as f32 / 2.0;

        pt2(x, y)
    }

    fn deforme_point_12(point: Point2) -> Point2 {
        let diff = pt2(point.x / NP as f32 * 2.0, point.y / NP as f32 * 2.0);
        let mut dh = diff.length();

        let mut angle = if diff.x == 0.0 {
            PI / 2.0 * sign(diff.y)
        } else {
            (diff.y / diff.x).atan() - PI * (sign(diff.x) - 1.0) / 2.0 * sign(diff.y)
        };

        dh = dh.powf(3.0);
        angle += PI / 18.0 * (6.0 * PI / dh).sin();

        let x = dh * angle.cos() * NP as f32 / 2.0;
        let y = 2.0 * dh * angle.sin() * NP as f32 / 2.0;

        pt2(x, y)
    }

    fn set_initials(&mut self) {
        if self.positions.len() != self.m + 1 {
            self.positions
                .resize_with(Self::calculate_positions(self.m).len(), Default::default);
        }
        if self.lengths.len() != self.n {
            self.lengths.resize_with(
                Self::calculate_lengths(self.m, self.n).len(),
                Default::default,
            );
        }
        if self.angles.len() != self.n {
            self.angles.resize_with(
                Self::calculate_angles(self.m, self.n).len(),
                Default::default,
            );
        }

        for ij in 0..=3 {
            let x = NP as f32 / 2.0 * (2.0 * ij as f32 * PI / 3.0).sin();
            let y = NP as f32 / 2.0 * (2.0 * ij as f32 * PI / 3.0).cos();
            self.positions[ij] = pt2(x, y);
        }
    }

    fn set_initials_2(&mut self) {
        self.m = 4;
        self.n = 4;

        self.positions.resize(self.m + 1, Default::default());
        for ij in 0..=4 {
            self.positions[ij] = pt2(
                NP as f32 / 2.0 * 5.0 / 6.0 * (2.0 * ij as f32 * PI / 4.0 + PI / 4.0).cos(),
                NP as f32 / 2.0 * 5.0 / 6.0 * (2.0 * ij as f32 * PI / 4.0 + PI / 4.0).sin(),
            );
        }

        self.lengths = vec![1.0 / (2.0 + 2.0 * (0.48 * PI).cos()); self.n];
        self.angles = vec![0.0, 0.48 * PI, -0.48 * PI, 0.0];
    }

    fn calculate_positions(m: usize) -> Vec<Point2> {
        let params = polygon::ParamsInner {
            k: m as u32,
            r: NP as f32 * 0.5,
            ad: 0.0,
        };
        let mut points = vec![];
        for i in 0..m {
            let point = params.calculate_point(i as u32);
            points.push(point);
        }
        points.push(points[0]);
        points
    }

    fn calculate_lengths(m: usize, n: usize) -> Vec<f32> {
        vec![1.0 / (m as f32); n]
    }

    fn calculate_angles(m: usize, n: usize) -> Vec<f32> {
        let mut angles = vec![0.0];

        for i in 1..(n - 1) {
            angles.push((PI / (m as f32)) * if i % 2 == 1 { 1.0 } else { -1.0 });
        }

        angles.push(0.0);

        angles
    }
}

impl Default for Params {
    fn default() -> Self {
        Self {
            inner: ParamsInner {
                deformation: Deformation::Program1,
                m: 3,
                n: 4,
                k: 4,
                positions: vec![],
                lengths: vec![1.0 / 3.0; 4],
                angles: vec![0.0, PI / 3.0, -PI / 3.0, 0.0],
            },
            calculate_shapes: Box::new(ParamsInner::calculate_shapes),
            ui_elements: Box::new(ParamsInner::ui_elements),
        }
    }
}
