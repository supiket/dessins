use nannou::geom::{pt2, Point2};
use nannou_egui::{egui, egui::Ui};
use std::f32::consts::PI;

pub struct DragonSettings {
    pub n: usize, // depth of recursion
    pub l0: f32,  // base length
    pub a0: f32,  // initial angle
}

impl DragonSettings {
    pub fn calculate_points(&self, rules: &[i32], p0: Point2) -> Vec<Point2> {
        let mut points = vec![p0];

        let mut p0 = p0;
        let mut p1 = p0;
        let mut p2 = p0;

        let mut current_angle = self.a0;

        let nn = 2_i32.pow(self.n as u32) - 1;

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

                let aa =
                    (rules[self.n - j] * 2 - 1) as f32 * ((((ii - 1) / 2) % 2) * 2 - 1) as f32 * PI
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
        let mut recalculate = false;

        ui.label("dragon n:");
        if ui.add(egui::Slider::new(&mut self.n, 2..=14)).changed() {
            recalculate = true;
        }

        recalculate
    }

    pub fn ui_a0(&mut self, ui: &mut Ui) -> bool {
        let mut recalculate = false;

        let mut a0 = self.a0 / PI;
        ui.label("polygon a0:");
        if ui
            .add(egui::Slider::new(&mut a0, -2.0..=2.0).suffix("π"))
            .changed()
        {
            recalculate = true;
        }
        self.a0 = a0 * PI;

        recalculate
    }
}
