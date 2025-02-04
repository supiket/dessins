use common::{
    chapter_6::{BipartiteSettings, InnerLine, OuterLine},
    ui_color, NP,
};
use nannou::prelude::*;
use nannou_egui::{self, egui, Egui};

struct Settings {
    bipartite: BipartiteSettings,
    color: Srgb<u8>,
}

struct Model {
    settings: Settings,
    egui: Egui,
    points: (OuterLine, InnerLine),
}

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
            bipartite: BipartiteSettings {
                n: 10,
                a: pt2((NP as f32) / -2.0, (NP as f32) / -2.0),
                b: pt2((NP as f32) / -2.0, (NP as f32) / 2.0),
                c: pt2((NP as f32) / 2.0, (NP as f32) / -2.0),
                d: pt2((NP as f32) / 2.0, (NP as f32) / 2.0),
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
            recalculate = model.settings.bipartite.ui_elements(ui);

            if let Some(color) = ui_color(ui) {
                model.settings.color = color;
            }
        });
    }

    if recalculate || model.points == Default::default() {
        calculate_points(model);
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);

    model.settings.bipartite.draw(
        &draw,
        model.settings.color,
        model.points.0.as_slice(),
        model.points.1.as_slice(),
    );

    draw.to_frame(app, &frame).unwrap();
    model.egui.draw_to_frame(&frame).unwrap();
}

fn calculate_points(model: &mut Model) {
    model.points = model.settings.bipartite.calculate_points();
}

fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    // let egui handle things like keyboard and mouse input.
    model.egui.handle_raw_event(event);
}

fn main() {
    nannou::app(model).update(update).run();
}
