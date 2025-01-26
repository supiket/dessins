use nannou::prelude::*;
use nannou_egui::{self, egui, Egui};

fn main() {
    nannou::app(model).update(update).run();
}

struct Settings {
    np: u32, // # elementary steps, i.e. resolution
    k: u32,  // # segments
    an: f32, // angle of two consecutive segments
    ra: f32, // ratio of the lengths of two consecutive segments
    aa: f32, // angle of the first segment with horizontal
    rr: f32, // length of the first segment
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
    let k = 200;
    let an = 15.0 * f32::PI() / 31.0;
    let ra = 0.98;
    let aa = 0.0;
    let rr = 0.80 * np as f32;
    let color = rgb(random(), random(), random());

    Model {
        egui,
        settings: Settings {
            np,
            k,
            an,
            ra,
            aa,
            rr,
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
        ui.add(egui::Slider::new(&mut settings.k, 1..=2500));

        ui.label("an:");
        ui.add(egui::Slider::new(&mut settings.an, 0.0..=f32::PI()));

        ui.label("ra:");
        ui.add(egui::Slider::new(&mut settings.ra, 0.0..=1.0));

        ui.label("aa:");
        ui.add(egui::Slider::new(&mut settings.aa, 0.0..=f32::PI()));

        let mut rr_multiplier = settings.rr / settings.np as f32;
        ui.label("rr multiplier:");
        ui.add(egui::Slider::new(&mut rr_multiplier, 0.0..=1.0));
        settings.rr = rr_multiplier * settings.np as f32;

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

    let mut aa = settings.aa;
    let mut rr = settings.rr;

    let mut x = (settings.np as f32 - settings.rr) / 2.0;
    let mut y = 0.0;

    let mut points = vec![];
    for i in 0..settings.k {
        x += rr * f32::cos(aa);
        y += rr * f32::sin(aa);

        let (x, y) = (x - 250.0, y - 250.0);
        let point = pt2(x, y);
        points.push(point);

        aa += settings.an;
        rr *= settings.ra;

        if i > 0 {
            let prev_point = points.get((i - 1) as usize).unwrap();
            draw.line()
                .start(*prev_point)
                .end(point)
                .color(settings.color)
                .weight(point_weight);
        }
    }

    draw.to_frame(app, &frame).unwrap();
    model.egui.draw_to_frame(&frame).unwrap();
}
