use common::{
    chapter_2::{Action, DessinShape, BIRD_FISH},
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
type Shape = Vec<Point2>;

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

    model
        .points
        .iter()
        .for_each(|shape| draw_exact(&draw, model.settings.color, shape));

    draw.to_frame(app, &frame).unwrap();
    model.egui.draw_to_frame(&frame).unwrap();
}

fn calculate_points(points: &mut Points) {
    *points = vec![vec![]];

    let mut bird_fish = DessinShape::new(BIRD_FISH);

    while let Action::Continue(read_point, newline) = bird_fish.calculate_point() {
        if newline {
            points.push(vec![])
        }

        let x = NP as f32 * (read_point.x + 0.5) / 15.0;
        let y = NP as f32 * (read_point.y + 0.5) / 15.0;

        let point = pt2(x, y);

        points[bird_fish.line_index].push(point);
    }
}

fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    model.egui.handle_raw_event(event);
}

fn main() {
    nannou::app(model).update(update).run();
}
