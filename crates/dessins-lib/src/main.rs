use dessins_lib::{
    draw_segment, match_calculate_shapes, match_ui_elements, ui_color, DesignParams, Shapes,
};
use nannou::prelude::*;
use nannou_egui::{egui, Egui};

pub struct Model {
    pub params: DesignParams,
    pub points: Shapes,
    pub egui: Egui,
    pub color: Srgb<u8>,
}

pub fn model(app: &App) -> Model {
    let window_id = app
        .new_window()
        .view(view)
        .raw_event(raw_window_event)
        .build()
        .unwrap();
    let window = app.window(window_id).unwrap();
    let egui = Egui::from_window(&window);
    let params = DesignParams::Polygon(dessins_lib::chapter_1::polygon::Params::default());

    Model {
        egui,
        params,
        points: Default::default(),
        color: rgb(random(), random(), random()),
    }
}

pub fn view(app: &App, model: &Model, frame: Frame) {
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

pub fn update(_app: &App, model: &mut Model, update: Update) {
    let mut recalculate = false;
    let mut new_design = Option::None;

    {
        model.egui.set_elapsed_time(update.since_start);
        let ctx = model.egui.begin_frame();

        egui::Window::new("designs").show(&ctx, |ui| {
            if ui.button("change design").clicked() {
                match &model.params {
                    DesignParams::Polygon(_params) => {
                        new_design = Some(DesignParams::Star(
                            dessins_lib::chapter_1::star::Params::default(),
                        ));
                    }
                    DesignParams::Star(_params) => {
                        new_design = Some(DesignParams::Polygon(
                            dessins_lib::chapter_1::polygon::Params::default(),
                        ));
                    }
                    _ => {}
                }
            }
        });

        egui::Window::new("params").show(&ctx, |ui| {
            recalculate = match_ui_elements(&mut model.params, ui);

            if let Some(color) = ui_color(ui) {
                model.color = color;
            }
        });
    }

    if let Some(new_design) = new_design {
        model.params = new_design;
        model.points = match_calculate_shapes(&model.params);
    } else if recalculate || model.points.is_empty() {
        model.points = match_calculate_shapes(&model.params);
    }
}

pub fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    model.egui.handle_raw_event(event);
}

fn main() {
    nannou::app(model).update(update).run();
}
