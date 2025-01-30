use nannou::prelude::*;
use nannou_egui::{self, egui, Egui};

fn main() {
    nannou::app(model).update(update).run();
}

struct Settings {
    np: u32,          // # elementary steps, i.e. resolution
    points: Vec<u32>, // points creating the horse figure
    color: Srgb<u8>,
}

struct Model {
    settings: Settings,
    egui: Egui,
}

fn model(app: &App) -> Model {
    let window_id = app
        .new_window()
        .view(view)
        .raw_event(raw_window_event)
        .build()
        .unwrap();
    let window = app.window(window_id).unwrap();

    let rect = app.window_rect();
    app.new_window().size(rect.w() as u32, rect.h() as u32 * 2);

    let egui = Egui::from_window(&window);

    // constants
    let np = 480;
    let points = vec![
        1000, 12, 12, 14, 8, 14, 4, 12, 2, 8, 2, 4, 4, 0, 10, 0, 20, 4, 26, 6, 28, 12, 28, 14, 26,
        14, 22, 12, 16, 12, 12, 20, 14, 24, 14, 28, 12, 28, 10, 26, 4, 28, 0, 36, 0, 38, 2, 40, 10,
        40, 22, 36, 26, 28, 26, 26, 22, 28, 14, 28, 12, 28, 14, 27, 18, 18, 18, 16, 20, 16, 18, 20,
        14, 16, 18, 12, 16, 1000, 16, 20, 16, 24, 20, 32, 20, 34, 20, 32, 12, 34, 12, 32, 10, 28,
        1000, 4, 26, 2, 28, 4, 30, 8, 30, 6, 32, 6, 34, 6, 32, 4, 32, 2, 30, 2, 28, 1000, 8, 30, 8,
        36, 10, 38, 1000, 4, 32, 4, 34, 8, 38, 6, 40, 6, 42, 8, 44, 10, 44, 10, 42, 12, 42, 12, 38,
        16, 36, 32, 36, 38, 40, 40, 44, 38, 42, 36, 46, 30, 48, 36, 48, 40, 44, 40, 56, 36, 62, 32,
        64, 24, 64, 18, 62, 16, 60, 16, 58, 18, 56, 24, 56, 22, 56, 20, 53, 28, 56, 22, 54, 28, 54,
        32, 52, 34, 48, 32, 52, 28, 48, 30, 46, 28, 44, 1000, 28, 48, 22, 48, 24, 48, 24, 52, 22,
        54, 18, 52, 18, 50, 20, 48, 12, 48, 16, 48, 18, 50, 16, 48, 16, 50, 18, 52, 16, 50, 16, 48,
        14, 46, 16, 44, 1000, 12, 48, 10, 44, 1000, 16, 46, 18, 44, 1000, 18, 46, 26, 46, 24, 46,
        24, 44, 22, 42, 20, 44, 20, 46, 1000, 22, 42, 22, 44, 24, 44, 1000, 28, 46, 26, 44, 1000,
        24, 54, 25, 52, 1000, 27, 52, 28, 54, 30, 52, 1000, 25, 49, 26, 50, 27, 49, 1000, 36, 38,
        40, 38, 42, 40, 48, 40, 48, 42, 50, 42, 52, 40, 50, 36, 48, 36, 48, 38, 48, 38, 48, 36, 46,
        34, 48, 36, 48, 26, 46, 24, 46, 32, 46, 30, 42, 30, 44, 28, 44, 26, 42, 24, 40, 26, 40, 32,
        42, 32, 28, 32, 30, 32, 32, 26, 1000, 44, 26, 44, 24, 46, 24, 1000, 42, 38, 44, 36, 44, 32,
        2000,
    ];

    // parameters
    let color = rgb(random(), random(), random());

    Model {
        egui,
        settings: Settings { np, points, color },
    }
}

fn update(_app: &App, model: &mut Model, update: Update) {
    let egui = &mut model.egui;
    let settings = &mut model.settings;

    egui.set_elapsed_time(update.since_start);
    let ctx = egui.begin_frame();

    egui::Window::new("settings").show(&ctx, |ui| {
        let clicked = ui.button("random color").clicked();
        if clicked {
            settings.color = rgb(random(), random(), random());
        }
    });
}

fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    // Let egui handle things like keyboard and mouse input.
    model.egui.handle_raw_event(event);
}

fn view(app: &App, model: &Model, frame: Frame) {
    let settings = &model.settings;

    let draw = app.draw();
    draw.background().color(BLACK);

    let point_weight = 2.0;
    let np = settings.np as f32;

    let mut points_index = 0;
    let mut points = vec![];
    let mut end_line = true;

    loop {
        let mut a = settings.points[points_index];
        points_index += 1;
        if a == 2000 {
            break;
        }
        if a == 1000 {
            a = settings.points[points_index];
            points_index += 1;
            end_line = true;
        }
        let b = settings.points[points_index];
        points_index += 1;
        let (a, b) = (a as f32, b as f32);

        let x = np * (a - 30.0) / 60.0;
        let y = np * (b - 30.0) / 60.0;

        let point = pt2(x, y);
        let last_point = points.last();

        if let Some(prev_point) = last_point {
            if !end_line {
                draw.line()
                    .start(*prev_point)
                    .end(point)
                    .color(settings.color)
                    .weight(point_weight);
            }
        }

        points.push(point);
        end_line = false;
    }

    draw.to_frame(app, &frame).unwrap();
    model.egui.draw_to_frame(&frame).unwrap();
}
