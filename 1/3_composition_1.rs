use common::draw_closed;
use nannou::prelude::*;
use nannou_egui::{self, egui, Egui};

struct Settings {
    np: u32, // # elementary steps, i.e. resolution
    k1: u32, // # stars
    r1: f32, // radius of the circle on which the centers of stars are
    a1: f32, // angle (in radians) of the vector CS with horizontal, where S is the center of the first star
    k: u32,  // # vertices of stars
    h: u32,  // # vertices to skip (clockwise) in a star before connecting two dots
    r: f32,  // radius of the circle on which the vertices of stars are
    ad: f32, // angle (in radians) of the vector corresponding to the first point of each star
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

    let np = 480;

    Model {
        egui,
        settings: Settings {
            np,
            k1: 5,
            r1: np as f32 * 0.27,
            a1: PI / 2.0,
            k: 25,
            h: 12,
            r: np as f32 * 0.22,
            ad: PI / 2.0,
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
            ui.label("k1:");
            if ui
                .add(egui::Slider::new(&mut model.settings.k1, 1..=100))
                .changed()
            {
                recalculate = true;
            }

            let mut r1 = model.settings.r1 / model.settings.np as f32;
            ui.label("r1:");
            if ui
                .add(
                    egui::Slider::new(&mut r1, 0.0..=1.0)
                        .suffix(format!("np (={})", model.settings.np)),
                )
                .changed()
            {
                recalculate = true;
            }
            model.settings.r1 = r1 * model.settings.np as f32;

            ui.label("a1:");
            if ui
                .add(egui::Slider::new(&mut model.settings.a1, 0.0..=PI))
                .changed()
            {
                recalculate = true;
            }

            ui.label("k:");
            if ui
                .add(egui::Slider::new(&mut model.settings.k, 4..=100))
                .changed()
            {
                recalculate = true;
            }

            ui.label("h:");
            if ui
                .add(egui::Slider::new(&mut model.settings.h, 1..=50))
                .changed()
            {
                recalculate = true;
            }

            let mut r = model.settings.r / model.settings.np as f32;
            ui.label("r:");
            if ui
                .add(
                    egui::Slider::new(&mut r, 0.0..=1.0)
                        .suffix(format!("np (={})", model.settings.np)),
                )
                .changed()
            {
                recalculate = true;
            }
            model.settings.r = r * model.settings.np as f32;

            let mut ad = model.settings.ad / PI;
            ui.label("ad:");
            if ui
                .add(egui::Slider::new(&mut ad, -1.0..=1.00).suffix("π"))
                .changed()
            {
                recalculate = true;
            }
            model.settings.ad = ad * PI;

            let clicked = ui.button("random color").clicked();
            if clicked {
                model.settings.color = rgb(random(), random(), random());
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

    for i in 0..settings.k1 {
        let angle = 2.0 * PI * i as f32 / settings.k1 as f32 + settings.a1;
        let cx = settings.r1 * f32::cos(angle);
        let cy = settings.r1 * f32::sin(angle);

        points.push(vec![]);

        for j in 0..settings.k {
            let angle = 2.0 * j as f32 * settings.h as f32 * PI / settings.k as f32 + settings.ad;
            let x = cx + settings.r * f32::cos(angle);
            let y = cy + settings.r * f32::sin(angle);
            points[i as usize].push(pt2(x, y));
        }
    }
}

fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    model.egui.handle_raw_event(event);
}

fn main() {
    nannou::app(model).update(update).run();
}
