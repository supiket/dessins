use common::{chapter_5::RotatingCurveSettings, draw_exact, ui_color, NP};
use nannou::prelude::*;
use nannou_egui::{self, egui, Egui};

struct Settings {
    curve: RotatingCurveSettings,
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
            curve: RotatingCurveSettings {
                n: 3000,
                t1: 0.5,
                t2: 30.0,
                r1: NP as f32 / 8.0,
                k1: 1,
                k2: 3,
                r2: NP as f32 * 0.27,
                h1: 1,
                h2: 1,
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

    let n = model.settings.curve.n as f32;
    let t1 = model.settings.curve.t1;
    let t2 = model.settings.curve.t2;
    let r1 = model.settings.curve.r1;
    let k1 = model.settings.curve.k1 as f32;
    let k2 = model.settings.curve.k2 as f32;
    let r2 = model.settings.curve.r2;
    let h1 = model.settings.curve.h1 as f32;
    let h2 = model.settings.curve.h2 as f32;

    for i in 0..=model.settings.curve.n {
        let i = i as f32;

        let an = 2.0 * PI * i / n;
        let c1 = (h1 * an * t1).cos();
        let s1 = (h2 * an * t1).sin();
        let c2 = (k1 * an * t2).cos();
        let s2 = (k2 * an * t2).sin();

        let x = r1 * c1 + r2 * (c1 * c2 - s1 * s2);
        let y = r1 * s1 + r2 * (s1 * c2 + c1 * s2);

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
