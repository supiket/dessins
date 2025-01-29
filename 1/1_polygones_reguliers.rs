use nannou::prelude::*;
use nannou_egui::{self, egui, Egui};

fn main() {
    nannou::app(model).update(update).run();
}

struct Settings {
    np: u32, // # elementary steps, i.e. resolution
    k: u32,  // # vertices
    r: f32,  // radius of the circle on which the vertices are
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

    // parameters
    let k = 3;
    let r = np as f32 * 0.45;
    let ad = 0 as f32;
    let color = rgb(random(), random(), random());

    Model {
        egui,
        settings: Settings {
            np,
            k,
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
        ui.add(egui::Slider::new(&mut settings.k, 3..=20));

        let mut r = settings.r / settings.np as f32;
        ui.label("r:");
        ui.add(egui::Slider::new(&mut r, 0.0..=1.0).suffix(format!("np (={})", settings.np)));
        settings.r = r * settings.np as f32;

        let mut ad = settings.ad / f32::PI();
        ui.label("ad:");
        ui.add(egui::Slider::new(&mut ad, -1.0..=1.00).suffix("π"));
        settings.ad = ad * f32::PI();

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
        let angle = (2.0 * i as f32 * f32::PI()) / settings.k as f32 + settings.ad;
        let x = settings.r * f32::cos(angle);
        let y = settings.r * f32::sin(angle);

        let point = pt2(x, y);
        let last_point = points.last();

        if let Some(prev_point) = last_point {
            draw.line()
                .start(*prev_point)
                .end(point)
                .color(settings.color)
                .weight(point_weight);
        }

        points.push(point);
    }

    draw.line()
        .start(*points.last().unwrap())
        .end(*points.first().unwrap())
        .color(settings.color)
        .weight(point_weight);

    draw.to_frame(app, &frame).unwrap();
    model.egui.draw_to_frame(&frame).unwrap();
}
