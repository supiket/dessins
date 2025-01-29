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

        let mut an = settings.an / f32::PI();
        ui.label("an:");
        ui.add(egui::Slider::new(&mut an, -1.0..=1.00).suffix("π"));
        settings.an = an * f32::PI();

        ui.label("ra:");
        ui.add(egui::Slider::new(&mut settings.ra, 0.0..=1.0));

        ui.label("aa:");
        ui.add(egui::Slider::new(&mut settings.aa, 0.0..=f32::PI()));

        let mut rr = settings.rr / settings.np as f32;
        ui.label("rr:");
        ui.add(egui::Slider::new(&mut rr, 0.0..=1.0).suffix(format!("np(={})", settings.np)));
        settings.rr = rr * settings.np as f32;

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
    let (w, h) = app.window_rect().w_h();
    draw.background().color(BLACK);

    let point_weight = 2.0;

    let mut current_length = settings.rr;
    let mut current_pos = pt2(-w / 4.0, -h / 4.0);
    let mut points = vec![current_pos];

    let mut min_x = 0.0;
    let mut max_x = 0.0;
    let mut min_y = 0.0;
    let mut max_y = 0.0;

    for i in 0..settings.k {
        let angle = settings.aa + i as f32 * settings.an;

        let dx = current_length * f32::cos(angle);
        let dy = current_length * f32::sin(angle);
        let point = pt2(current_pos.x + dx, current_pos.y + dy);

        // update bounds
        min_x = min_x.min(point.x);
        max_x = max_x.max(point.x);
        min_y = min_y.min(point.y);
        max_y = max_y.max(point.y);

        points.push(point);
        current_pos = point;
        current_length *= settings.ra;
    }

    // calculate center offset
    let center_offset_x = (min_x + max_x) / 2.0;
    let center_offset_y = (min_y + max_y) / 2.0;

    // draw centered segments
    for i in 0..points.len() - 1 {
        let start = pt2(points[i].x - center_offset_x, points[i].y - center_offset_y);
        let end = pt2(
            points[i + 1].x - center_offset_x,
            points[i + 1].y - center_offset_y,
        );

        draw.line()
            .start(start)
            .end(end)
            .color(settings.color)
            .weight(point_weight);
    }

    draw.to_frame(app, &frame).unwrap();
    model.egui.draw_to_frame(&frame).unwrap();
}
