use common::draw_exact;
use nannou::prelude::*;
use nannou_egui::{self, egui, Egui};

struct Settings {
    np: u32,          // # elementary steps, i.e. resolution
    points: Vec<u32>, // points creating the smurf figure
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
                1000, 12, 12, 14, 8, 14, 4, 12, 2, 8, 2, 4, 4, 0, 10, 0, 20, 4, 26, 6, 28, 12, 28,
                14, 26, 14, 22, 12, 16, 12, 12, 20, 14, 24, 14, 28, 12, 28, 10, 26, 4, 28, 0, 36,
                0, 38, 2, 40, 10, 40, 22, 36, 26, 28, 26, 26, 22, 28, 14, 28, 12, 28, 14, 27, 18,
                18, 18, 16, 20, 16, 18, 20, 14, 16, 18, 12, 16, 1000, 16, 20, 16, 24, 20, 32, 20,
                34, 20, 32, 12, 34, 12, 32, 10, 28, 1000, 4, 26, 2, 28, 4, 30, 8, 30, 6, 32, 6, 34,
                6, 32, 4, 32, 2, 30, 2, 28, 1000, 8, 30, 8, 36, 10, 38, 1000, 4, 32, 4, 34, 8, 38,
                6, 40, 6, 42, 8, 44, 10, 44, 10, 42, 12, 42, 12, 38, 16, 36, 32, 36, 38, 40, 40,
                44, 38, 42, 36, 46, 30, 48, 36, 48, 40, 44, 40, 56, 36, 62, 32, 64, 24, 64, 18, 62,
                16, 60, 16, 58, 18, 56, 24, 56, 22, 56, 20, 53, 28, 56, 22, 54, 28, 54, 32, 52, 34,
                48, 32, 52, 28, 48, 30, 46, 28, 44, 1000, 28, 48, 22, 48, 24, 48, 24, 52, 22, 54,
                18, 52, 18, 50, 20, 48, 12, 48, 16, 48, 18, 50, 16, 48, 16, 50, 18, 52, 16, 50, 16,
                48, 14, 46, 16, 44, 1000, 12, 48, 10, 44, 1000, 16, 46, 18, 44, 1000, 18, 46, 26,
                46, 24, 46, 24, 44, 22, 42, 20, 44, 20, 46, 1000, 22, 42, 22, 44, 24, 44, 1000, 28,
                46, 26, 44, 1000, 24, 54, 25, 52, 1000, 27, 52, 28, 54, 30, 52, 1000, 25, 49, 26,
                50, 27, 49, 1000, 36, 38, 40, 38, 42, 40, 48, 40, 48, 42, 50, 42, 52, 40, 50, 36,
                48, 36, 48, 38, 48, 38, 48, 36, 46, 34, 48, 36, 48, 26, 46, 24, 46, 32, 46, 30, 42,
                30, 44, 28, 44, 26, 42, 24, 40, 26, 40, 32, 42, 32, 28, 32, 30, 32, 32, 26, 1000,
                44, 26, 44, 24, 46, 24, 1000, 42, 38, 44, 36, 44, 32, 2000,
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
    let np = settings.np as f32;

    for i in 0..7 {
        points.push(vec![vec![]]);
        index += 1;

        let mut points_index = 0;
        let mut polyline_index = 0;

        let k = (0.5).pow(i) as f32;

        loop {
            let mut a = settings.points[points_index];
            points_index += 1;
            if a == 2000 {
                break;
            }
            if a == 1000 {
                a = settings.points[points_index];
                points_index += 1;
                polyline_index += 1;
                points[index as usize].push(vec![]);
            }
            let b = settings.points[points_index];
            let (a, b) = (a as f32, b as f32);
            points_index += 1;

            let x = np / 100.0 * a * k + 0.5 * np - np * k;
            let y = np / 100.0 * b * k - 0.5 * np;

            points[index as usize][polyline_index as usize].push(pt2(x, y));
        }
    }
}

fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    model.egui.handle_raw_event(event);
}

fn main() {
    nannou::app(model).update(update).run();
}
