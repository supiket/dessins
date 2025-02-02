use common::{
    chapter_2::{Action, DessinShape, HORSE},
    draw_exact, ui_color, NP,
};
use nannou::prelude::*;
use nannou_egui::{self, egui, Egui};

struct Settings {
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
        calculate_points(&mut model.points);
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

fn calculate_points(points: &mut Points) {
    *points = vec![];

    let n = 4;

    for i in -n..=n {
        for j in -n..=n {
            let mut shape = vec![vec![]];

            let mut horse = DessinShape::new(HORSE);

            while let Action::Continue(read_point, newline) = horse.calculate_point() {
                if newline {
                    shape.push(vec![])
                }

                let xx = (read_point.x + j as f32 * 20.0 - 20.0) / 100.0;
                let yy = (read_point.y + i as f32 * 20.0 - 20.0) / 100.0;

                let x = abs(xx).powf(0.7) * if xx < 0.0 { -1.0 } else { 1.0 } * NP as f32 / 2.0;
                let y = abs(yy).powf(0.7) * if yy < 0.0 { -1.0 } else { 1.0 } * NP as f32 / 2.0;

                let point = pt2(x, y);

                shape[horse.line_index].push(point);
            }

            points.push(shape);
        }
    }
}

fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    model.egui.handle_raw_event(event);
}

fn main() {
    nannou::app(model).update(update).run();
}
