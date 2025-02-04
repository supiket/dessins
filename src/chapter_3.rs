use crate::{add_float_slider_pi, add_number_slider};
use nannou::geom::{pt2, Point2};
use nannou_egui::egui::Ui;
use std::f32::consts::PI;

pub struct DragonSettings {
    pub n: u32,  // depth of recursion
    pub l0: f32, // base length
    pub a0: f32, // initial angle
}

impl DragonSettings {
    pub fn calculate_points(&self, rules: &[i32], p0: Point2) -> Vec<Point2> {
        let mut points = vec![p0];

        let mut p0 = p0;
        let mut p1 = p0;
        let mut p2 = p0;

        let mut current_angle = self.a0;

        let nn = 2_i32.pow(self.n) - 1;

        fn step_points(p0: &mut Point2, p1: &mut Point2, p2: &mut Point2, step: Point2) {
            *p0 = *p1;
            *p1 = *p2;
            *p2 += step;
        }

        for i in 0..=nn {
            if i == 0 {
                step_points(
                    &mut p0,
                    &mut p1,
                    &mut p2,
                    pt2(self.l0 * current_angle.cos(), self.l0 * current_angle.sin()),
                );
            } else {
                let mut ii = i;
                let mut j = 0;

                while ii % 2 == 0 {
                    ii /= 2;
                    j += 1;
                }

                let aa = (rules[self.n as usize - j] * 2 - 1) as f32
                    * ((((ii - 1) / 2) % 2) * 2 - 1) as f32
                    * PI
                    / 2.0;
                current_angle += aa;

                step_points(
                    &mut p0,
                    &mut p1,
                    &mut p2,
                    pt2(self.l0 * current_angle.cos(), self.l0 * current_angle.sin()),
                );
            }

            points.push((p0 + pt2(3.0, 3.0) * p1) / pt2(4.0, 4.0));
            points.push((p2 + pt2(3.0, 3.0) * p1) / pt2(4.0, 4.0));
        }

        points
    }

    pub fn ui_n(&mut self, ui: &mut Ui) -> bool {
        add_number_slider(ui, "dragon n", &mut self.n, 2..=14)
    }

    pub fn ui_a0(&mut self, ui: &mut Ui) -> bool {
        add_float_slider_pi(ui, "dragon a0", &mut self.a0, -2.0..=2.0)
    }
}
