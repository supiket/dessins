use common::{chapter_5::OrbitalCurveSettings, draw_exact, ui_color, NP};
use nannou::prelude::*;
use nannou_egui::{self, egui, Egui};

struct Settings {
    curve: OrbitalCurveSettings,
    color: Srgb<u8>,
}

struct Model {
    settings: Settings,
    egui: Egui,
    points: Points,
}

type Points = Vec<Point2>;

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
            curve: OrbitalCurveSettings {
                n: 2000,
                t1: 1,
                t2: 100,
                r1: NP as f32 * 0.27,
                k1: 1,
                k2: 1,
            },
            color: rgb(random(), random(), random()),
        },
        points: Default::default(),
    }
}

fn update(_app: &App, model: &mut Model, update: Update) {
    let mut recalculate = false;

    {
        model.egui.set_elapsed_time(update.since_start);
        let ctx = model.egui.begin_frame();

        egui::Window::new("settings").show(&ctx, |ui| {
            if model.settings.curve.ui_elements(ui) {
                recalculate = true;
            }

            if let Some(color) = ui_color(ui) {
                model.settings.color = color;
            }
        });
    }

    if recalculate || model.points.is_empty() {
        calculate_points(model);
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);

    draw_exact(&draw, model.settings.color, model.points.as_slice());

    draw.to_frame(app, &frame).unwrap();
    model.egui.draw_to_frame(&frame).unwrap();
}

fn calculate_points(model: &mut Model) {
    model.points = vec![];

    let np = NP as f32;
    let n = model.settings.curve.n as f32;
    let t1 = model.settings.curve.t1 as f32;
    let t2 = model.settings.curve.t2 as f32;
    let r1 = model.settings.curve.r1 as f32;
    let k1 = model.settings.curve.k1 as f32;
    let k2 = model.settings.curve.k2 as f32;

    for i in 0..=model.settings.curve.n {
        let i = i as f32;
        let r2 = np * 0.23 * (0.5 * (2.0 * i * PI / n * 3.0).cos() + 0.5);
        let a1 = 2.0 * PI * i / n * t1;
        let a2 = 2.0 * PI * i / n * t2;

        let x = r1 * (k1 * a1).cos() + r2 * a2.cos();
        let y = 1.4 * (r1 * (k2 * a1).sin() + r2 * a2.sin());

        model.points.push(pt2(x, y));
    }
}

fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    // let egui handle things like keyboard and mouse input.
    model.egui.handle_raw_event(event);
}

fn main() {
    nannou::app(model).update(update).run();
}
