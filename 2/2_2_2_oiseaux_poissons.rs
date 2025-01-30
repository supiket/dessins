use nannou::prelude::*;
use nannou_egui::{self, egui, Egui};

fn main() {
    nannou::app(model).update(update).run();
}

struct Settings {
    np: u32,          // # elementary steps, i.e. resolution
    points: Vec<f32>, // points creating the horse figure
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
        1000.0, 0.0, 0.0, 2.0, 0.0, 4.0, 1.0, 4.0, 2.0, 3.0, 2.0, 2.0, 3.0, 4.0, 5.0, 4.0, 6.0,
        2.0, 5.0, 2.0, 6.0, -1.0, 5.0, -2.0, 3.0, -1.0, 2.0, -2.0, 2.0, -3.0, 3.0, -4.0, 3.0, -5.0,
        2.0, -4.0, 2.0, 0.0, 0.0, 1000.0, -5.0, 2.0, -5.0, 1.0, -7.0, -1.0, -6.0, -2.0, -5.0, -2.0,
        -5.0, -3.0, -2.0, -2.0, -2.0, -3.0, 0.0, -2.0, 1.0, -1.0, 2.0, -1.0, 3.0, -2.0, 4.0, -2.0,
        3.0, -1.0, 4.0, 1.0, 1000.0, 2.0, 5.0, 0.0, 4.0, 0.0, 2.0, 1000.0, -2.0, 1.0, -5.0, 1.0,
        -4.0, -1.0, -3.0, 0.0, -3.0, -1.0, -4.0, -1.0, -5.0, -2.0, 0.0, -2.0, 1000.0, -7.0, -1.0,
        -6.0, -1.0, 1000.0, -4.0, 2.5, -4.0, 2.8, -4.3, 2.8, -4.3, 2.5, -4.0, 2.5, 1000.0, -5.0,
        0.0, -5.5, 0.0, -5.5, 0.5, -5.0, 0.5, -5.0, 0.0, 2000.0,
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

    for i in 1..=4 {
        for j in 1..=4 {
            let mut points_index = 0;
            let mut points = vec![];
            let mut end_line = true;

            loop {
                let mut a = settings.points[points_index];
                points_index += 1;
                if a == 2000.0 {
                    break;
                }
                if a == 1000.0 {
                    a = settings.points[points_index];
                    points_index += 1;
                    end_line = true;
                }
                let b = settings.points[points_index];
                points_index += 1;

                let x = np * (b - 22.5 + 4.0 * i as f32 + 4.0 * j as f32) / 45.0;
                let y = np * (a - 7.5 - 5.0 * i as f32 + 9.0 * j as f32) / 45.0;

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
        }
    }

    draw.to_frame(app, &frame).unwrap();
    model.egui.draw_to_frame(&frame).unwrap();
}
