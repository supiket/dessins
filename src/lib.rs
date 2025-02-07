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

pub struct NoParamsInner();

pub struct NoParams {
    pub inner: NoParamsInner,
    pub calculate_shapes: Box<dyn Fn(&NoParamsInner) -> Shapes>,
    pub ui_elements: Box<dyn Fn(&mut NoParamsInner, &mut Ui) -> bool>,
}

pub enum DesignParams {
    No(NoParams),
    Polygon(chapter_1::polygon::Params),
    Star(chapter_1::star::Params),
    Composition1(chapter_1::composition_1::Params),
    Composition2(chapter_1::composition_2::Params),
    Jolygon(chapter_1::jolygon::Params),
    Dragon(chapter_3::Params),
    Fractal(chapter_4::Params),
    Orbital(chapter_5::orbital::Params),
    Rotating(chapter_5::rotating::Params),
    Spiral(chapter_5::spiral::Params),
    Bipartite(chapter_6::bipartite::Params),
    LinearModulo(chapter_6::linear_modulo::Params),
    LinearSticks(chapter_6::linear_sticks::Params),
    SimpleFractal(chapter_7::Params),
}

pub struct Model {
    pub params: DesignParams,
    pub points: Shapes,
    pub egui: Egui,
    pub color: Srgb<u8>,
}

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

pub fn model(params: DesignParams, app: &App) -> Model {
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
        params,
        points: Default::default(),
        color: rgb(random(), random(), random()),
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
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

pub fn update(model: &mut Model, update: Update) {
    let mut recalculate = false;

    {
        model.egui.set_elapsed_time(update.since_start);
        let ctx = model.egui.begin_frame();

        egui::Window::new("params").show(&ctx, |ui| {
            recalculate = match &mut model.params {
                DesignParams::No(params) => (params.ui_elements)(&mut params.inner, ui),
                DesignParams::Polygon(params) => (params.ui_elements)(&mut params.inner, ui),

                DesignParams::Star(params) => (params.ui_elements)(&mut params.inner, ui),
                DesignParams::Composition1(params) => (params.ui_elements)(&mut params.inner, ui),
                DesignParams::Composition2(params) => (params.ui_elements)(&mut params.inner, ui),
                DesignParams::Jolygon(params) => (params.ui_elements)(&mut params.inner, ui),
                DesignParams::Dragon(params) => (params.ui_elements)(&mut params.inner, ui),
                DesignParams::Fractal(params) => (params.ui_elements)(&mut params.inner, ui),
                DesignParams::Orbital(params) => (params.ui_elements)(&mut params.inner, ui),
                DesignParams::Rotating(params) => (params.ui_elements)(&mut params.inner, ui),
                DesignParams::Spiral(params) => (params.ui_elements)(&mut params.inner, ui),
                DesignParams::Bipartite(params) => (params.ui_elements)(&mut params.inner, ui),
                DesignParams::LinearModulo(params) => (params.ui_elements)(&mut params.inner, ui),
                DesignParams::LinearSticks(params) => (params.ui_elements)(&mut params.inner, ui),
                DesignParams::SimpleFractal(params) => (params.ui_elements)(&mut params.inner, ui),
            };

            if let Some(color) = ui_color(ui) {
                model.color = color;
            }
        });
    }

    if recalculate || model.points.is_empty() {
        model.points = match &model.params {
            DesignParams::No(params) => (params.calculate_shapes)(&params.inner),
            DesignParams::Polygon(params) => (params.calculate_shapes)(&params.inner),
            DesignParams::Star(params) => (params.calculate_shapes)(&params.inner),
            DesignParams::Composition1(params) => (params.calculate_shapes)(&params.inner),
            DesignParams::Composition2(params) => (params.calculate_shapes)(&params.inner),
            DesignParams::Jolygon(params) => (params.calculate_shapes)(&params.inner),
            DesignParams::Dragon(params) => (params.calculate_shapes)(&params.inner),
            DesignParams::Fractal(params) => (params.calculate_shapes)(&params.inner),
            DesignParams::Orbital(params) => (params.calculate_shapes)(&params.inner),
            DesignParams::Rotating(params) => (params.calculate_shapes)(&params.inner),
            DesignParams::Spiral(params) => (params.calculate_shapes)(&params.inner),
            DesignParams::Bipartite(params) => (params.calculate_shapes)(&params.inner),
            DesignParams::LinearModulo(params) => (params.calculate_shapes)(&params.inner),
            DesignParams::LinearSticks(params) => (params.calculate_shapes)(&params.inner),
            DesignParams::SimpleFractal(params) => (params.calculate_shapes)(&params.inner),
        };
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

pub fn no_ui_elements(_inner: &mut NoParamsInner, _ui: &mut Ui) -> bool {
    false
}

fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    model.egui.handle_raw_event(event);
}
