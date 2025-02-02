use common::draw_exact;
use nannou::prelude::*;
use nannou_egui::{self, egui, Egui};

struct Settings {
    np: u32,          // # elementary steps, i.e. resolution
    points: Vec<f32>, // points creating the bird-fish figure
    color: Srgb<u8>,
}

struct Model {
    settings: Settings,
    points: Points,
    egui: Egui,
}

type Points = Vec<Shape>;
type Shape = Vec<Line>;
type Line = Vec<Point2>;

fn model(app: &App) -> Model {
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
            np: 480,
            points: vec![
                1000.0, 0.0, 0.0, 2.0, 0.0, 4.0, 1.0, 4.0, 2.0, 3.0, 2.0, 2.0, 3.0, 4.0, 5.0, 4.0,
                6.0, 2.0, 5.0, 2.0, 6.0, -1.0, 5.0, -2.0, 3.0, -1.0, 2.0, -2.0, 2.0, -3.0, 3.0,
                -4.0, 3.0, -5.0, 2.0, -4.0, 2.0, 0.0, 0.0, 1000.0, -5.0, 2.0, -5.0, 1.0, -7.0,
                -1.0, -6.0, -2.0, -5.0, -2.0, -5.0, -3.0, -2.0, -2.0, -2.0, -3.0, 0.0, -2.0, 1.0,
                -1.0, 2.0, -1.0, 3.0, -2.0, 4.0, -2.0, 3.0, -1.0, 4.0, 1.0, 1000.0, 2.0, 5.0, 0.0,
                4.0, 0.0, 2.0, 1000.0, -2.0, 1.0, -5.0, 1.0, -4.0, -1.0, -3.0, 0.0, -3.0, -1.0,
                -4.0, -1.0, -5.0, -2.0, 0.0, -2.0, 1000.0, -7.0, -1.0, -6.0, -1.0, 1000.0, -4.0,
                2.5, -4.0, 2.8, -4.3, 2.8, -4.3, 2.5, -4.0, 2.5, 1000.0, -5.0, 0.0, -5.5, 0.0,
                -5.5, 0.5, -5.0, 0.5, -5.0, 0.0, 2000.0,
            ],
            color: rgb(random(), random(), random()),
        },
        points: Default::default(),
    }
}

fn update(_app: &App, model: &mut Model, update: Update) {
    {
        model.egui.set_elapsed_time(update.since_start);
        let ctx = model.egui.begin_frame();

        egui::Window::new("settings").show(&ctx, |ui| {
            let clicked = ui.button("random color").clicked();
            if clicked {
                model.settings.color = rgb(random(), random(), random());
            }
        });
    }

    if model.points.is_empty() {
        calculate_points(&model.settings, &mut model.points);
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);

    model.points.iter().for_each(|shape| {
        shape
            .iter()
            .for_each(|line| draw_exact(&draw, model.settings.color, line))
    });

    draw.to_frame(app, &frame).unwrap();
    model.egui.draw_to_frame(&frame).unwrap();
}

fn calculate_points(settings: &Settings, points: &mut Points) {
    *points = vec![];
    let mut index: i32 = -1;

    for i in 1..=4 {
        for j in 1..=4 {
            points.push(vec![vec![]]);
            index += 1;

            let mut points_index = 0;
            let mut polyline_index = 0;
            loop {
                let mut a = settings.points[points_index];
                points_index += 1;
                if a == 2000.0 {
                    break;
                }
                if a == 1000.0 {
                    a = settings.points[points_index];
                    points_index += 1;
                    polyline_index += 1;
                    points[index as usize].push(vec![]);
                }
                let b = settings.points[points_index];
                points_index += 1;

                let x = settings.np as f32 * (b - 22.5 + 4.0 * i as f32 + 4.0 * j as f32) / 45.0;
                let y = settings.np as f32 * (a - 7.5 - 5.0 * i as f32 + 9.0 * j as f32) / 45.0;

                points[index as usize][polyline_index as usize].push(pt2(x, y));
            }
        }
    }
}

fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    model.egui.handle_raw_event(event);
}

fn main() {
    nannou::app(model).update(update).run();
}
