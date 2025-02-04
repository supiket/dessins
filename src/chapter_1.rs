use crate::{draw_closed, ui_color, Shapes};
use nannou::prelude::*;
use nannou_egui::{
    egui::{self, Ui},
    Egui,
};

pub mod jolygon;
pub mod polygon;
pub mod star;

pub struct Model<S> {
    settings: S,
    calculate_shapes: Box<dyn Fn(&S) -> Shapes>,
    points: Shapes,
    pub egui: Egui,
    color: Srgb<u8>,
}

pub fn model<S>(calculate_shapes: Box<dyn Fn(&S) -> Shapes>, settings: S, app: &App) -> Model<S>
where
    S: 'static,
{
    let window_id = app
        .new_window()
        .view(view::<S>)
        .raw_event(raw_window_event::<S>)
        .build()
        .unwrap();
    let window = app.window(window_id).unwrap();
    let egui = Egui::from_window(&window);

    Model {
        egui,
        calculate_shapes,
        settings,
        color: rgb(random(), random(), random()),
        points: Default::default(),
    }
}

pub fn update<S>(model: &mut Model<S>, update: Update, elements: impl Fn(&mut S, &mut Ui) -> bool) {
    let mut recalculate = false;

    {
        model.egui.set_elapsed_time(update.since_start);
        let ctx = model.egui.begin_frame();

        egui::Window::new("settings").show(&ctx, |ui| {
            recalculate = (elements)(&mut model.settings, ui);

            if let Some(color) = ui_color(ui) {
                model.color = color;
            }
        });
    }

    if recalculate || model.points.is_empty() {
        model.points = (model.calculate_shapes)(&model.settings);
    }
}

fn view<Settings>(app: &App, model: &Model<Settings>, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);

    model.points.iter().for_each(|shape| {
        shape
            .iter()
            .for_each(|line| draw_closed(&draw, model.color, line))
    });

    draw.to_frame(app, &frame).unwrap();
    model.egui.draw_to_frame(&frame).unwrap();
}

fn raw_window_event<S>(
    _app: &App,
    model: &mut Model<S>,
    event: &nannou::winit::event::WindowEvent,
) {
    model.egui.handle_raw_event(event);
}
