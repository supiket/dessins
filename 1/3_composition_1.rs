use common::{
    chapter_1::{calculate_polygon, calculate_stars, PolygonSettings, StarSettings},
    draw_closed, ui_color, NP,
};
use nannou::prelude::*;
use nannou_egui::{self, egui, Egui};

struct Settings {
    polygon: PolygonSettings,
    star: StarSettings,
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
            polygon: PolygonSettings {
                k: 5,
                r: NP as f32 * 0.27,
                ad: PI / 2.0,
            },
            star: StarSettings {
                k: 25,
                h: 12,
                r: NP as f32 * 0.22,
                ad: PI / 2.0,
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
            if model.settings.polygon.ui_elements(ui) {
                recalculate = true;
            }

            if model.settings.star.ui_elements(ui) {
                recalculate = true;
            }

            if let Some(color) = ui_color(ui) {
                model.settings.color = color;
            }
        });
    }

    if recalculate || model.points.is_empty() {
        calculate_points(&model.settings, &mut model.points);
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);

    model
        .points
        .iter()
        .for_each(|shape| draw_closed(&draw, model.settings.color, shape));

    draw.to_frame(app, &frame).unwrap();
    model.egui.draw_to_frame(&frame).unwrap();
}

fn calculate_points(settings: &Settings, points: &mut Points) {
    *points = vec![];

    for i in 0..settings.polygon.k {
        let polygon_point = calculate_polygon(&settings.polygon, i);

        points.push(vec![]);

        for j in 0..settings.star.k {
            let star_point = calculate_stars(&settings.star, j);
            let point = star_point + polygon_point;
            points[i as usize].push(point);
        }
    }
}

fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    model.egui.handle_raw_event(event);
}

fn main() {
    nannou::app(model).update(update).run();
}
