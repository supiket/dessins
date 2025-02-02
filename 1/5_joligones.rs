use common::{draw_exact, ui_color, NP};
use nannou::prelude::*;
use nannou_egui::{self, egui, Egui};

struct Settings {
    k: u32,  // # segments
    an: f32, // angle of two consecutive segments
    ra: f32, // ratio of the lengths of two consecutive segments
    aa: f32, // angle of the first segment with horizontal
    rr: f32, // length of the first segment
    initial_pos: Point2,
    color: Srgb<u8>,
}

struct Model {
    settings: Settings,
    points: Points,
    egui: Egui,
}

type Points = Shape;
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

    let (w, h) = window.rect().w_h();

    Model {
        egui,
        settings: Settings {
            k: 200,
            an: 15.0 * PI / 31.0,
            ra: 0.98,
            aa: 0_f32,
            rr: 0.80 * NP as f32,
            initial_pos: pt2(-w / 4.0, -h / 4.0),
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
            ui.label("k:");
            if ui
                .add(egui::Slider::new(&mut model.settings.k, 1..=2500))
                .changed()
            {
                recalculate = true;
            }

            let mut an = model.settings.an / PI;
            ui.label("an:");
            if ui
                .add(egui::Slider::new(&mut an, -1.0..=1.00).suffix("π"))
                .changed()
            {
                recalculate = true;
            }
            model.settings.an = an * PI;

            ui.label("ra:");
            if ui
                .add(egui::Slider::new(&mut model.settings.ra, 0.0..=1.0))
                .changed()
            {
                recalculate = true;
            }

            ui.label("aa:");
            if ui
                .add(egui::Slider::new(&mut model.settings.aa, 0.0..=PI))
                .changed()
            {
                recalculate = true;
            }

            let mut rr = model.settings.rr / NP as f32;
            ui.label("rr:");
            if ui
                .add(egui::Slider::new(&mut rr, 0.0..=1.0).suffix(format!("np(={})", NP)))
                .changed()
            {
                recalculate = true;
            }
            model.settings.rr = rr * NP as f32;

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

    draw_exact(&draw, model.settings.color, &model.points);

    draw.to_frame(app, &frame).unwrap();
    model.egui.draw_to_frame(&frame).unwrap();
}

fn calculate_points(settings: &Settings, points: &mut Points) {
    *points = vec![];
    let _points: Vec<Point2> = vec![];

    let mut current_length = settings.rr;
    let mut current_pos = settings.initial_pos;
    let mut _points = vec![current_pos];

    let mut min_x = 0.0;
    let mut max_x = 0.0;
    let mut min_y = 0.0;
    let mut max_y = 0.0;

    for i in 0..settings.k {
        let angle = settings.aa + i as f32 * settings.an;

        let dx = current_length * f32::cos(angle);
        let dy = current_length * f32::sin(angle);
        let d = pt2(dx, dy);
        let point = current_pos + d;

        // update bounds
        min_x = min_x.min(point.x);
        max_x = max_x.max(point.x);
        min_y = min_y.min(point.y);
        max_y = max_y.max(point.y);

        _points.push(point);
        current_pos = point;
        current_length *= settings.ra;
    }

    // calculate center offset
    let center_offset_x = (min_x + max_x) / 2.0;
    let center_offset_y = (min_y + max_y) / 2.0;

    // make segments centered
    for point in _points {
        points.push(point - pt2(center_offset_x, center_offset_y));
    }
}

fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    model.egui.handle_raw_event(event);
}

fn main() {
    nannou::app(model).update(update).run();
}
