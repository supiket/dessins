use common::{
    chapter_2::{Action, DessinShape, HORSE},
    draw_exact, ui_color, NP,
};
use nannou::prelude::*;
use nannou_egui::{self, egui, Egui};

struct Settings {
    count: u32, // # repetitions of pattern
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
            count: 16,
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
            if let Some(color) = ui_color(ui) {
                model.settings.color = color;
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

    for i in 0..settings.count {
        let mut shape = vec![vec![]];

        let mut horse = DessinShape::new(HORSE);

        let an = 2.0 * i as f32 * PI / 6.0 + PI / 12.0;
        let co = an.cos();
        let si = an.sin();
        let r = pow(0.87, i as usize);

        while let Action::Continue(read_point, newline) = horse.calculate_point() {
            if newline {
                shape.push(vec![])
            }

            let x_ = 0.15 + read_point.x / 110.0;
            let y_ = 0.15 + read_point.y / 110.0;
            let x = NP as f32 * (r * (co * x_ - si * y_));
            let y = NP as f32 * (r * (si * x_ + co * y_));
            let point = pt2(x, y);

            shape[horse.line_index].push(point);
        }

        points.push(shape);
    }
}

fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    model.egui.handle_raw_event(event);
}

fn main() {
    nannou::app(model).update(update).run();
}
