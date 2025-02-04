use crate::{add_float_slider, add_float_slider_np, add_number_slider, draw_exact, ui_color};
use nannou::prelude::*;
use nannou_egui::{
    egui::{self, Ui},
    Egui,
};
use std::f32::consts::PI;

pub struct CurveSettings {
    pub n: u32,  // # segments
    pub t1: f32, // # times the planet turns around the sun
    pub t2: f32, // # times the satellite turns around the planet
    pub r1: f32, // radius of the planet's curve
    pub k1: u32, // elliptic parameter of the planet's curve
    pub k2: u32, // elliptic parameter of the planet's curve
    pub r2: f32, // radius of the satellite's curve
    pub h1: u32, // elliptic parameter of the satellite's curve
    pub h2: u32, // elliptic parameter of the satellite's curve
    pub s_eq: Box<dyn Fn(SParams) -> f32>,
}

pub struct SParams {
    pub i: f32,
    pub n: f32,
}

pub struct Settings {
    curve: CurveSettings,
    color: Srgb<u8>,
}

pub struct Model {
    settings: Settings,
    egui: Egui,
    points: Points,
}

pub type Points = Vec<Point2>;

pub fn model(curve: CurveSettings, app: &App) -> Model {
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
        settings: Settings {
            curve,
            color: rgb(random(), random(), random()),
        },
        points: Default::default(),
    }
}

pub fn update(_app: &App, model: &mut Model, update: Update) {
    let mut recalculate = false;

    {
        model.egui.set_elapsed_time(update.since_start);
        let ctx = model.egui.begin_frame();

        egui::Window::new("settings").show(&ctx, |ui| {
            recalculate = model.settings.curve.ui_elements(ui);

            if let Some(color) = ui_color(ui) {
                model.settings.color = color;
            }
        });
    }

    if recalculate || model.points.is_empty() {
        model.points = model.settings.curve.calculate_points();
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);

    draw_exact(&draw, model.settings.color, model.points.as_slice());

    draw.to_frame(app, &frame).unwrap();
    model.egui.draw_to_frame(&frame).unwrap();
}

impl CurveSettings {
    pub fn ui_elements(&mut self, ui: &mut Ui) -> bool {
        add_number_slider(ui, "curve n", &mut self.n, 1000..=6000)
            || add_float_slider(ui, "curve t1", &mut self.t1, 0.5..=600.0)
            || add_float_slider(ui, "curve t2", &mut self.t2, 0.5..=600.0)
            || add_float_slider_np(ui, "curve r1", &mut self.r1, 0.0..=1.0)
            || add_float_slider_np(ui, "curve r2", &mut self.r2, 0.0..=1.0)
            || add_number_slider(ui, "curve k1", &mut self.k1, 1..=4)
            || add_number_slider(ui, "curve k2", &mut self.k2, 1..=4)
            || add_number_slider(ui, "curve h1", &mut self.h1, 1..=4)
            || add_number_slider(ui, "curve h2", &mut self.h2, 1..=4)
    }

    pub fn calculate_points(&self) -> Vec<Point2> {
        let mut points = vec![];

        let n = self.n as f32;
        let t1 = self.t1;
        let t2 = self.t2;
        let r1 = self.r1;
        let k1 = self.k1 as f32;
        let k2 = self.k2 as f32;
        let r2 = self.r2;
        let h1 = self.h1 as f32;
        let h2 = self.h2 as f32;

        for i in 0..=self.n {
            let i = i as f32;

            let s = (self.s_eq)(SParams { i, n });
            let an = 2.0 * PI * i / n;
            let c1 = (h1 * an * t1).cos();
            let s1 = (h2 * an * t1).sin();
            let c2 = s * (k1 * an * t2).cos();
            let s2 = s * (k2 * an * t2).sin();

            let x = r1 * c1 + r2 * (c1 * c2 - s1 * s2);
            let y = r1 * s1 + r2 * (s1 * c2 + c1 * s2);

            points.push(pt2(x, y));
        }

        points
    }
}

fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    // let egui handle things like keyboard and mouse input.
    model.egui.handle_raw_event(event);
}
