use crate::{
    adjustable_dessin::{update_from_reflect, AdjustableDessin},
    adjustable_variable::types::{Context, F32Variant, VecF32, VecPt2, U32},
    shapes::{sign, Segment, Shape, Shapes, NP},
};
use nannou::prelude::*;

pub type OuterSegment = Segment;
pub type InnerSegment = Segment;

#[derive(Clone, Debug, PartialEq, Reflect)]
#[reflect(Default)]
pub struct Deformed {
    #[reflect(ignore)]
    pub deformation: Deformation,
    #[reflect(ignore)]
    pub m: U32, // # of segments in starting curve
    #[reflect(ignore)]
    pub n: U32, // # of sub-segments per segment
    pub k: U32, // depth
    pub positions: VecPt2,
    pub lengths: VecF32,
    pub angles: VecF32,
}

#[derive(Clone, Debug, PartialEq)]
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

impl Deformation {
    fn update(&mut self, ui: &mut egui::Ui) -> bool {
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

impl Deformed {
    pub fn calculate_shapes(&mut self) -> Shapes {
        if self.positions.get_value().len() != self.m.get_value() as usize + 1
            || self.lengths.get_value().len() != self.n.get_value() as usize
            || self.angles.get_value().len() != self.n.get_value() as usize
        {
            match self.deformation {
                Deformation::Program9
                | Deformation::Program10
                | Deformation::Program11
                | Deformation::Program12 => {
                    if self.deformation != Deformation::Program9
                        && self.deformation != Deformation::Program10
                        && self.deformation != Deformation::Program11
                        && self.deformation != Deformation::Program12
                    {
                        self.set_initials_2();
                    }
                }
                _ => {
                    if self.deformation != Deformation::Program1
                        && self.deformation != Deformation::Program2
                        && self.deformation != Deformation::Program3
                        && self.deformation != Deformation::Program4
                        && self.deformation != Deformation::Program5
                        && self.deformation != Deformation::Program6
                        && self.deformation != Deformation::Program7
                        && self.deformation != Deformation::Program8
                    {
                        self.set_initials();
                    }
                }
            }
        }

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

        let mut shapes = Shapes::new();
        let mut shape = Shape::new();

        for ii in 0..self.m.get_value() as usize {
            let mut segment = Segment::new();

            let source = self.positions.get_value()[ii].get_value();
            let destination = self.positions.get_value()[ii + 1].get_value();
            let diff = destination - source;

            let mut point = source;
            segment.push(deforme_point(point));

            let angle = if diff.x == 0.0 {
                PI / 2.0 * sign(diff.y)
            } else {
                (diff.y / diff.x).atan()
            } + if diff.x < 0.0 { PI } else { 0.0 };

            let length = diff.length();

            for i in 0..(self.n.get_value() as usize).pow(self.k.get_value()) {
                let mut current_length = length;
                let mut current_angle = angle;
                let mut t1 = i;
                if self.k.get_value() as usize != 0 {
                    for j in (0..self.k.get_value()).rev() {
                        let r = (self.n.get_value() as usize).pow(j);
                        let t2 = t1 / r;
                        current_angle += self.angles.get_value()[t2].get_value();
                        current_length *= self.lengths.get_value()[t2].get_value();
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

        pt2(y, x)
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

        pt2(y, x)
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

        pt2(y, x)
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

        pt2(y, x)
    }

    fn set_initials(&mut self) {
        self.m.set_value(3);
        self.n.set_value(4);

        let multiplier = NP as f32 / 2.0;

        self.positions = VecPt2::new(
            (0..=3)
                .map(|ij| {
                    let angle = 2.0 * ij as f32 * PI / 3.0;
                    pt2(multiplier * angle.sin(), multiplier * angle.cos())
                })
                .collect(),
        );

        self.lengths = VecF32::new(
            vec![1.0 / 3.0; self.n.get_value() as usize],
            F32Variant::None(0.0..=1.0),
        );
        self.angles = VecF32::new(vec![0.0, 1.0 / 3.0, -1.0 / 3.0, 0.0], F32Variant::Angle);
    }

    fn set_initials_2(&mut self) {
        self.m.set_value(4);
        self.n.set_value(4);

        let multiplier = NP as f32 / 2.0 * 5.0 / 6.0;

        self.positions = VecPt2::new(
            (0..=4)
                .map(|ij| {
                    let angle = 2.0 * ij as f32 * PI / 4.0 + PI / 4.0;
                    pt2(multiplier * angle.cos(), multiplier * angle.sin())
                })
                .collect(),
        );
        self.lengths = VecF32::new(
            vec![1.0 / (2.0 + 2.0 * (0.48 * PI).cos()); self.n.get_value() as usize],
            F32Variant::None(0.0..=1.0),
        );
        self.angles = VecF32::new(vec![0.0, 0.48, -0.48, 0.0], F32Variant::Angle);
    }
}

impl AdjustableDessin for Deformed {
    fn update_variables(
        &mut self,
        ui: &mut egui::Ui,
        osc_ctx: &Context,
        time: Time<Virtual>,
    ) -> bool {
        let mut changed = false;

        let deformation = self.deformation.clone();
        let deformation_changed = self.deformation.update(ui);

        if deformation_changed {
            match self.deformation {
                Deformation::Program9
                | Deformation::Program10
                | Deformation::Program11
                | Deformation::Program12 => {
                    if deformation != Deformation::Program9
                        && deformation != Deformation::Program10
                        && deformation != Deformation::Program11
                        && deformation != Deformation::Program12
                    {
                        self.set_initials_2();
                    }
                }
                _ => {
                    if deformation != Deformation::Program1
                        && deformation != Deformation::Program2
                        && deformation != Deformation::Program3
                        && deformation != Deformation::Program4
                        && deformation != Deformation::Program5
                        && deformation != Deformation::Program6
                        && deformation != Deformation::Program7
                        && deformation != Deformation::Program8
                    {
                        self.set_initials();
                    }
                }
            }
        }

        changed |= deformation_changed;
        ui.separator();
        changed |= update_from_reflect(self, ui, osc_ctx, time);
        ui.separator();
        changed
    }
}

impl Default for Deformed {
    fn default() -> Self {
        let mut self_ = Self {
            deformation: Deformation::Program1,
            m: U32::new(3, 1..=4),
            n: U32::new(4, 2..=5),
            k: U32::new(4, 1..=6),
            positions: VecPt2::new(vec![]),
            lengths: VecF32::new(vec![], F32Variant::None(0.0..=1.0)),
            angles: VecF32::new(vec![], F32Variant::Angle),
        };
        self_.set_initials();
        self_
    }
}
