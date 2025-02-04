use common::{
    add_float_slider, add_float_slider_np, add_float_slider_pi, add_number_slider,
    chapter_1::{self, Model},
    Shapes, NP,
};
use nannou::prelude::*;
use nannou_egui::{self, egui::Ui};

struct Settings {
    k: u32,  // # segments
    an: f32, // angle of two consecutive segments
    ra: f32, // ratio of the lengths of two consecutive segments
    aa: f32, // angle of the first segment with horizontal
    rr: f32, // length of the first segment
    initial_pos: Point2,
}

fn model(app: &App) -> Model<Settings> {
    let (w, h) = app.window_rect().w_h();

    let settings = Settings {
        k: 200,
        an: 15.0 * PI / 31.0,
        ra: 0.98,
        aa: 0_f32,
        rr: 0.80 * NP as f32,
        initial_pos: pt2(-w / 4.0, -h / 4.0),
    };

    chapter_1::model(Box::new(calculate_shapes), settings, app)
}

fn ui_elements(settings: &mut Settings, ui: &mut Ui) -> bool {
    add_number_slider(ui, "k", &mut settings.k, 1..=2500)
        || add_float_slider_pi(ui, "an", &mut settings.an, -1.0..=1.0)
        || add_float_slider(ui, "ra", &mut settings.ra, 0.0..=1.0)
        || add_float_slider_pi(ui, "aa", &mut settings.aa, 0.0..=1.0)
        || add_float_slider_np(ui, "rr", &mut settings.rr, 0.0..=1.0)
}

fn update(_app: &App, model: &mut Model<Settings>, update: Update) {
    chapter_1::update(model, update, ui_elements);
}

fn calculate_shapes(settings: &Settings) -> Shapes {
    let mut current_length = settings.rr;
    let mut current_pos = settings.initial_pos;
    let mut line = vec![current_pos];

    let mut min_x = 0.0;
    let mut max_x = 0.0;
    let mut min_y = 0.0;
    let mut max_y = 0.0;

    for i in 0..settings.k {
        let angle = settings.aa + i as f32 * settings.an;

        let dx = current_length * angle.cos();
        let dy = current_length * angle.sin();
        let d = pt2(dx, dy);
        let point = current_pos + d;

        // update bounds
        min_x = min_x.min(point.x);
        max_x = max_x.max(point.x);
        min_y = min_y.min(point.y);
        max_y = max_y.max(point.y);

        line.push(point);
        current_pos = point;
        current_length *= settings.ra;
    }

    // calculate center offset
    let center_offset_x = (min_x + max_x) / 2.0;
    let center_offset_y = (min_y + max_y) / 2.0;

    // make segments centered
    for point in &mut line {
        *point -= pt2(center_offset_x, center_offset_y);
    }

    vec![vec![line]]
}

fn main() {
    nannou::app(model).update(update).run();
}
