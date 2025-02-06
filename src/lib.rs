use nannou::prelude::*;
use nannou_egui::{
    egui::{self, emath::Numeric, Ui},
    Egui,
};
use std::{f32::consts::PI, ops::RangeInclusive};

pub mod chapter_1;
pub mod chapter_2;
pub mod chapter_3;
pub mod chapter_4;
pub mod chapter_5;
pub mod chapter_6;
pub mod chapter_7;

pub const NP: usize = 480; // # elementary steps, i.e. resolution
pub const WEIGHT: f32 = 1.0; // point weight

pub type Shapes = Vec<Shape>;
pub type Shape = Vec<Segment>;
pub type Segment = Vec<Point2>;

pub struct Model<P> {
    pub params: P,
    pub calculate_shapes: Box<dyn Fn(&P) -> Shapes>,
    pub points: Shapes,
    pub egui: Egui,
    pub color: Srgb<u8>,
}

pub struct NoParams();

pub fn draw_segment(draw: &Draw, color: Srgb<u8>, points: &[Point2]) {
    if points.len() < 2 {
        return;
    }

    for i in 0..points.len() - 1 {
        let start = points[i];
        let end = points[i + 1];

        draw.line()
            .start(start)
            .end(end)
            .color(color)
            .weight(WEIGHT);
    }
}

pub fn model<P: 'static>(
    calculate_shapes: Box<dyn Fn(&P) -> Shapes>,
    params: P,
    app: &App,
) -> Model<P> {
    let window_id = app
        .new_window()
        .view(view::<P>)
        .raw_event(raw_window_event::<P>)
        .build()
        .unwrap();
    let window = app.window(window_id).unwrap();
    let egui = Egui::from_window(&window);

    Model {
        calculate_shapes,
        egui,
        params,
        points: Default::default(),
        color: rgb(random(), random(), random()),
    }
}

fn view<P>(app: &App, model: &Model<P>, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);

    model.points.iter().for_each(|shape| {
        shape
            .iter()
            .for_each(|segment| draw_segment(&draw, model.color, segment))
    });

    draw.to_frame(app, &frame).unwrap();
    model.egui.draw_to_frame(&frame).unwrap();
}

pub fn update<P>(model: &mut Model<P>, update: Update, elements: impl Fn(&mut P, &mut Ui) -> bool) {
    let mut recalculate = false;

    {
        model.egui.set_elapsed_time(update.since_start);
        let ctx = model.egui.begin_frame();

        egui::Window::new("params").show(&ctx, |ui| {
            recalculate = (elements)(&mut model.params, ui);

            if let Some(color) = ui_color(ui) {
                model.color = color;
            }
        });
    }

    if recalculate || model.points.is_empty() {
        model.points = (model.calculate_shapes)(&model.params);
    }
}

pub fn ui_color(ui: &mut Ui) -> Option<Srgb<u8>> {
    let clicked = ui.button("random color").clicked();
    if clicked {
        Some(rgb(random(), random(), random()))
    } else {
        None
    }
}

pub fn add_number_slider<T: Numeric>(
    ui: &mut Ui,
    label: &str,
    value: &mut T,
    range: RangeInclusive<T>,
) -> bool {
    ui.label(label);
    ui.add(egui::Slider::new(&mut *value, range)).changed()
}

pub fn add_float_slider(
    ui: &mut Ui,
    label: &str,
    value: &mut f32,
    range: RangeInclusive<f32>,
) -> bool {
    ui.label(label);
    ui.add(
        egui::Slider::new(&mut *value, range).custom_parser(|str| evalexpr::eval_float(str).ok()),
    )
    .changed()
}

pub fn add_float_slider_np_position(ui: &mut Ui, label: &str, value: &mut f32) -> bool {
    add_float_slider_np(ui, label, value, -0.5..=0.5)
}

pub fn add_float_slider_np_length(ui: &mut Ui, label: &str, value: &mut f32) -> bool {
    add_float_slider_np(ui, label, value, 0.0..=1.0)
}

fn add_float_slider_np(
    ui: &mut Ui,
    label: &str,
    value: &mut f32,
    range: RangeInclusive<f32>,
) -> bool {
    ui.label(label);
    let mut val = *value / NP as f32;

    let recalculate = ui
        .add(
            egui::Slider::new(&mut val, range)
                .custom_parser(|str| evalexpr::eval_float(str).ok())
                .suffix(format!("np (={})", NP)),
        )
        .changed();

    *value = val * NP as f32;

    recalculate
}

pub fn add_float_slider_pi(ui: &mut Ui, label: &str, value: &mut f32) -> bool {
    ui.label(label);
    let mut val = *value / PI;

    let recalculate = ui
        .add(
            egui::Slider::new(&mut val, -PI..=PI)
                .custom_parser(|str| evalexpr::eval_float(str).ok())
                .suffix("π"),
        )
        .changed();

    *value = val * PI;

    recalculate
}

pub fn no_ui_elements(_params: &mut NoParams, _ui: &mut Ui) -> bool {
    false
}

fn raw_window_event<P>(
    _app: &App,
    model: &mut Model<P>,
    event: &nannou::winit::event::WindowEvent,
) {
    model.egui.handle_raw_event(event);
}
