use common::{
    add_float_slider, add_number_slider,
    chapter_1::{calculate_polygon, calculate_stars, PolygonSettings, StarSettings},
    draw_closed, ui_color, NP,
};
use nannou::prelude::*;
use nannou_egui::{self, egui, Egui};

struct Settings {
    polygon: PolygonSettings,
    star: StarSettings,
    n: u32,  // # stars
    rr: f32, // reduction coefficient from one star to the next & the distance between the center of the spiral and the center of successive stars
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
            n: 32,
            rr: 0.9,
            polygon: PolygonSettings {
                k: 8,
                r: NP as f32 * 0.36,
                ad: 0_f32,
            },
            star: StarSettings {
                k: 16,
                h: 5,
                r: NP as f32 * 0.14,
                ad: 0_f32,
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
            recalculate = add_number_slider(ui, "n", &mut model.settings.n, 1..=100)
                || add_float_slider(ui, "rr", &mut model.settings.rr, 0.0..=1.0)
                || model.settings.polygon.ui_elements(ui)
                || model.settings.star.ui_elements(ui);

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

    for i in 0..settings.n {
        let r2 = settings.polygon.r * settings.rr.powi(i as i32);
        let r3 = settings.star.r * settings.rr.powi(i as i32);

        let mut polygon_settings = settings.polygon.clone();
        polygon_settings.r = r2;
        let polygon_point = calculate_polygon(&polygon_settings, i);

        points.push(vec![]);

        for j in 0..settings.star.k {
            let mut star_settings = settings.star.clone();
            star_settings.r = r3;
            let star_point = calculate_stars(&star_settings, j);
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
