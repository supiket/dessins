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
        1000, 10, 10, 8, 12, 9, 16, 12, 17, 13, 18, 14, 20, 1000, 13, 18, 12, 19, 9, 21, 9, 20, 10,
        19, 9, 17, 7, 20, 8, 22, 12, 22, 1000, 12, 20, 12, 22, 13, 26, 16, 31, 18, 31, 19, 32,
        1000, 16, 31, 14, 31, 14, 32, 1000, 14, 31, 10, 30, 12, 31, 10, 32, 10, 34, 11, 34, 11, 33,
        10, 33, 1000, 12, 32, 13, 31, 1000, 10, 34, 16, 36, 1000, 16, 35, 16, 37, 18, 35, 17, 34,
        1000, 17, 36, 20, 36, 22, 32, 19, 26, 1000, 20, 36, 22, 36, 22, 34, 24, 32, 24, 30, 19, 26,
        18, 23, 21, 22, 21, 24, 30, 30, 34, 31, 36, 31, 33, 26, 32, 22, 28, 22, 27, 20, 29, 17, 30,
        19, 29, 20, 29, 21, 32, 19, 33, 18, 32, 17, 29, 16, 28, 12, 30, 10, 21, 4, 21, 2, 18, 3,
        19, 6, 24, 10, 24, 12, 22, 14, 22, 16, 23, 17, 1000, 22, 16, 17, 16, 16, 17, 17, 18, 1000,
        16, 17, 16, 16, 10, 14, 10, 12, 12, 11, 10, 10, 1000, 21, 21, 22, 24, 30, 30, 1000, 24, 24,
        34, 28, 1000, 25, 23, 33, 26, 1000, 25, 21, 27, 20, 1000, 23, 21, 24, 19, 1000, 27, 20, 22,
        19, 22, 21, 1000, 22, 19, 21, 20, 1000, 13, 34, 15, 35, 16, 34, 16, 33, 1000, 15, 35, 15,
        34, 16, 34, 15, 34, 15, 35, 1000, 24, 12, 26, 10, 19, 5, 19, 3, 1000, 28, 22, 25, 22, 2000,
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

    let n = 4;
    for i in -n..=n {
        for j in -abs(i)..=abs(i) {
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

                let xx = (a + j as f32 * 20.0 - 20.0) / 100.0;
                let yy = (b + i as f32 * 20.0 - 20.0) / 100.0;

                let x = xx * 0.5 * np;
                let y = yy * 0.5 * np;

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
