use crate::{add_float_slider_pi, add_number_slider, draw_exact, ui_color, Shapes};
use nannou::geom::{pt2, Point2};
use nannou::prelude::*;
use nannou_egui::{
    egui::{self, Ui},
    Egui,
};
use std::f32::consts::PI;

pub struct DragonSettings {
    pub n: u32,  // depth of recursion
    pub l0: f32, // base length
    pub a0: f32, // initial angle
    pub p0: Point2,
}

pub struct Model {
    pub settings: DragonSettings,
    pub egui: Egui,
    pub rules: Vec<i32>, // turning rules
    points: Shapes,
    color: Srgb<u8>,
}

pub fn model(settings: DragonSettings, rules: &[i32], app: &App) -> Model {
    let window_id = app
        .new_window()
        .view(view)
        .raw_event(raw_window_event)
        .build()
        .unwrap();
    let window = app.window(window_id).unwrap();
    let egui = Egui::from_window(&window);

    Model {
        egui,
        settings,
        rules: rules.to_vec(),
        points: Default::default(),
        color: rgb(random(), random(), random()),
    }
}

pub fn update(
    model: &mut Model,
    update: Update,
    elements: impl Fn(&mut DragonSettings, &mut Ui) -> bool,
) {
    let mut recalculate = false;

    {
        model.egui.set_elapsed_time(update.since_start);
        let ctx = model.egui.begin_frame();

        egui::Window::new("settings").show(&ctx, |ui| {
            recalculate = (elements)(&mut model.settings, ui);

            if let Some(color) = ui_color(ui) {
                model.color = color;
            }
        });
    }

    if recalculate || model.points.is_empty() {
        model.points = model.settings.calculate_shapes(&model.rules);
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);

    model.points.iter().for_each(|shape| {
        shape
            .iter()
            .for_each(|line| draw_exact(&draw, model.color, line))
    });

    draw.to_frame(app, &frame).unwrap();
    model.egui.draw_to_frame(&frame).unwrap();
}

fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    model.egui.handle_raw_event(event);
}

impl DragonSettings {
    pub fn calculate_shapes(&self, rules: &[i32]) -> Shapes {
        let p0 = self.p0;

        let mut line = vec![p0];

        let mut p0 = p0;
        let mut p1 = p0;
        let mut p2 = p0;

        let mut current_angle = self.a0;

        let nn = 2_i32.pow(self.n) - 1;

        fn step_line(p0: &mut Point2, p1: &mut Point2, p2: &mut Point2, step: Point2) {
            *p0 = *p1;
            *p1 = *p2;
            *p2 += step;
        }

        for i in 0..=nn {
            if i == 0 {
                step_line(
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

                step_line(
                    &mut p0,
                    &mut p1,
                    &mut p2,
                    pt2(self.l0 * current_angle.cos(), self.l0 * current_angle.sin()),
                );
            }

            line.push((p0 + pt2(3.0, 3.0) * p1) / pt2(4.0, 4.0));
            line.push((p2 + pt2(3.0, 3.0) * p1) / pt2(4.0, 4.0));
        }

        vec![vec![line]]
    }

    pub fn ui_n(&mut self, ui: &mut Ui) -> bool {
        add_number_slider(ui, "dragon n", &mut self.n, 2..=14)
    }

    pub fn ui_a0(&mut self, ui: &mut Ui) -> bool {
        add_float_slider_pi(ui, "dragon a0", &mut self.a0, -2.0..=2.0)
    }
}
