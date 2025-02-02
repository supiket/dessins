use common::{
    chapter_2::{Action, DessinShape, LION},
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

    for i in 0..5 {
        for j in 0..3 {
            let mut shape = vec![vec![]];

            let mut lion = DessinShape::new(LION);

            while let Action::Continue(read_point, newline) = lion.calculate_point() {
                if newline {
                    shape.push(vec![])
                }

                let x = NP as f32
                    * (-18.0
                        + (1.0 - 2.0 * (i % 2) as f32) * (7.0 - read_point.x)
                        + 4.0
                        + 14.0 * j as f32)
                    / 50.0;
                let y = NP as f32
                    * (-20.5
                        + (1.0 - 2.0 * (j % 2) as f32) * (4.5 - read_point.y)
                        + 4.0
                        + 9.0 * i as f32)
                    / 50.0;

                let point = pt2(x, y);

                shape[lion.line_index].push(point);
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
