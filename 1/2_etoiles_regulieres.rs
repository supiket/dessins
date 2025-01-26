use nannou::prelude::*;
use nannou_egui::{self, egui, Egui};

fn main() {
    nannou::app(model).update(update).run();
}

struct Settings {
    np: u32, // # elementary steps, i.e. resolution
    k: u32,  // # vertices
    h: u32,  // # vertices to skip (clockwise) before connecting two dots
    cx: f32, // x-coordinate of the circle C on which the vertices are
    cy: f32, // y-coordinate of the circle C on which the vertices are
    r: f32,  // radius of the circle C on which the vertices are
    ad: f32, // angle (in radians) of the vector CS with horizontal, where S is the first vertex
    color: Srgb<u8>,
}

struct Model {
    settings: Settings,
    egui: Egui,
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

    // constants
    let np = 480;
    let cx = np as f32 / 2.0;
    let cy = np as f32 / 2.0;

    // parameters
    let k = 5;
    let h = 3;
    let r = np as f32 * 0.45;
    let ad = f32::PI() / 2.0;
    let color = rgb(random(), random(), random());

    Model {
        egui,
        settings: Settings {
            np,
            cx,
            cy,
            k,
            h,
            r,
            ad,
            color,
        },
    }
}

fn update(_app: &App, model: &mut Model, update: Update) {
    let egui = &mut model.egui;
    let settings = &mut model.settings;

    egui.set_elapsed_time(update.since_start);
    let ctx = egui.begin_frame();

    egui::Window::new("settings").show(&ctx, |ui| {
        ui.label("k:");
        ui.add(egui::Slider::new(&mut settings.k, 5..=100));

        ui.label("h:");
        ui.add(egui::Slider::new(&mut settings.h, 3..=50));

        let mut r_multiplier = settings.r / settings.np as f32;
        ui.label("r multiplier:");
        ui.add(egui::Slider::new(&mut r_multiplier, 0.0..=1.0));
        settings.r = r_multiplier * settings.np as f32;

        ui.label("ad:");
        ui.add(egui::Slider::new(&mut settings.ad, 0.0..=f32::PI()));

        let clicked = ui.button("random color").clicked();
        if clicked {
            settings.color = rgb(random(), random(), random());
        }
    });
}

fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    // Let egui handle things like keyboard and mouse input.
    model.egui.handle_raw_event(event);
}

fn view(app: &App, model: &Model, frame: Frame) {
    let settings = &model.settings;

    let draw = app.draw();
    draw.background().color(BLACK);

    let point_weight = 2.0;

    let mut points = vec![];
    for i in 0..settings.k {
        let x = settings.cx
            + settings.r
                * f32::cos(
                    (2.0 * i as f32 * settings.h as f32 * f32::PI()) / settings.k as f32
                        + settings.ad,
                );
        let y = settings.cy
            + settings.r
                * f32::sin(
                    (2.0 * i as f32 * settings.h as f32 * f32::PI()) / settings.k as f32
                        + settings.ad,
                );
        let (x, y) = (x - 250.0, y - 250.0);
        let point = pt2(x, y);
        points.push(point);
        if i > 0 {
            let prev_point = points.get(i as usize - 1).unwrap();
            draw.line()
                .start(*prev_point)
                .end(point)
                .color(settings.color)
                .weight(point_weight);
        }
    }

    draw.line()
        .start(*points.last().unwrap())
        .end(*points.first().unwrap())
        .color(settings.color)
        .weight(point_weight);

    draw.to_frame(app, &frame).unwrap();
    model.egui.draw_to_frame(&frame).unwrap();
}
