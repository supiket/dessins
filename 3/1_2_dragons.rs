use common::{chapter_3::DragonSettings, draw_exact, ui_color, NP};
use nannou::prelude::*;
use nannou_egui::{self, egui, Egui};

struct Settings {
    dragon: DragonSettings,
    color: Srgb<u8>,
}

struct Model {
    settings: Settings,
    egui: Egui,
    rules: Vec<i32>, // turning rules
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
            dragon: DragonSettings {
                n: 10,
                l0: Default::default(),
                a0: -PI / 2.0,
            },
            color: rgb(random(), random(), random()),
        },
        rules: Default::default(),
        points: Default::default(),
    }
}

fn update(_app: &App, model: &mut Model, update: Update) {
    let mut recalculate = false;

    {
        model.egui.set_elapsed_time(update.since_start);
        let ctx = model.egui.begin_frame();

        egui::Window::new("settings").show(&ctx, |ui| {
            if model.settings.dragon.ui_n(ui) {
                recalculate = true;
            }

            if model.settings.dragon.ui_a0(ui) {
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
    model.settings.dragon.l0 =
        NP as f32 / (2.0_f32.sqrt().powf(model.settings.dragon.n as f32)) * 0.9;
    let mut rules = vec![0; model.settings.dragon.n + 1];
    for i in (0..=model.settings.dragon.n).step_by(3) {
        rules[i] = 1;
    }
    model.rules = rules;
    let p0 = pt2(-(NP as f32) / 4.0, -(NP as f32) / 3.0);

    let points = model.settings.dragon.calculate_points(&model.rules, p0);
    model.points = points;
}

fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    // let egui handle things like keyboard and mouse input.
    model.egui.handle_raw_event(event);
}

fn main() {
    nannou::app(model).update(update).run();
}
