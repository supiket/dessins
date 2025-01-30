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
        1000.0, -2.5, 0.0, -2.0, 1.0, -1.0, 2.0, 0.0, 7.0, 1.0, 7.0, 2.0, 8.0, 2.0, 11.0, 3.0,
        14.0, 3.5, 13.5, 2.5, 11.0, 2.5, 9.0, 1000.0, 3.5, 13.5, 4.0, 13.0, 3.0, 11.0, 3.0, 9.0,
        3.0, 11.0, 4.0, 13.0, 5.0, 12.0, 3.5, 11.0, 3.5, 9.0, 3.5, 11.0, 5.0, 12.0, 5.0, 11.0, 4.0,
        10.0, 4.0, 9.0, 8.0, 9.0, 7.0, 11.0, 8.0, 13.0, 10.0, 14.0, 12.0, 13.0, 13.0, 11.0, 12.0,
        11.0, 11.0, 10.0, 12.0, 8.0, 13.0, 7.0, 14.0, 2.0, 15.0, 2.0, 16.0, 1.0, 16.0, 0.0, 12.0,
        0.0, 12.0, 2.0, 11.0, 5.0, 11.5, 6.0, 11.0, 5.0, 9.0, 3.0, 9.0, 2.0, 10.0, 1.0, 10.0, 0.0,
        6.0, 0.0, 7.0, 2.0, 8.0, 6.0, 7.0, 2.0, 6.0, 4.0, 4.0, 5.0, 5.0, 7.0, 4.0, 8.0, 5.0, 7.0,
        4.0, 5.0, 2.0, 4.0, 1.0, 2.0, 2.0, 2.0, 3.0, 1.0, 2.5, 0.0, -2.5, 0.0, 1000.0, 6.0, 4.0,
        7.5, 3.5, 1000.0, 12.0, 11.0, 10.0, 10.5, 9.0, 10.5, 1000.0, 12.5, 12.0, 12.0, 12.0, 11.0,
        11.5, 12.0, 12.0, 12.0, 12.5, 11.5, 12.5, 10.5, 13.0, 10.0, 13.0, 10.0, 13.5, 10.5, 13.5,
        10.5, 13.0, 11.5, 12.5, 12.0, 12.5, 12.0, 13.0, 1000.0, 7.5, 12.0, 8.5, 12.0, 8.5, 11.5,
        2000.0,
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

    for i in 0..5 {
        for j in 0..3 {
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

                let x = np
                    * (-18.0 + (1.0 - 2.0 * (i % 2) as f32) * (7.0 - a) + 4.0 + 14.0 * j as f32)
                    / 50.0;
                let y = np
                    * (-20.5 + (1.0 - 2.0 * (j % 2) as f32) * (4.5 - b) + 4.0 + 9.0 * i as f32)
                    / 50.0;

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
