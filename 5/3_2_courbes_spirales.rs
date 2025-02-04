use common::{chapter_5::SpiralCurveSettings, draw_exact, ui_color, NP};
use nannou::prelude::*;
use nannou_egui::{self, egui, Egui};

struct Settings {
    curve: SpiralCurveSettings,
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
            curve: SpiralCurveSettings {
                n: 3000,
                t: 60,
                r: 0.1,
                l: 0.1,
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
            recalculate = model.settings.curve.ui_elements(ui);

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
    let t = model.settings.curve.t as f32;
    let r = model.settings.curve.r;
    let l = model.settings.curve.l;

    for i in 0..=model.settings.curve.n {
        let i = i as f32;

        let rr = l.powf(i / n);
        let an = 2.0 * PI * i / n;

        let x = rr * (t * an).cos();
        let y = rr * r * (t * an).sin();

        let co = an.cos();
        let si = an.sin();

        let xx = x * co - y * si;
        let yy = x * si + y * co;

        let x = xx * np / 2.0;
        let y = yy * np / 2.0;

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
