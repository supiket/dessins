use dessins_lib::{
    draw_segment, match_calculate_shapes, match_ui_elements, ui::ui_color, DesignParams, Shapes,
};
use nannou::prelude::*;
use nannou_egui::{
    egui::{self, Ui},
    Egui,
};

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
    let params = DesignParams::Dragon(dessins_lib::chapter_3::Params::default());

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

        egui::TopBottomPanel::top("designs").show(&ctx, |ui| {
            ui.horizontal(|ui| {
                new_design = design_buttons(&model.params, ui);
            })
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
        model.points = match_calculate_shapes(&mut model.params);
    } else if recalculate || model.points.is_empty() {
        model.points = match_calculate_shapes(&mut model.params);
    }
}

fn design_buttons(params: &DesignParams, ui: &mut Ui) -> Option<DesignParams> {
    let mut new_design = Option::None;
    if let Some(new) = dessins_lib::chapter_1::polygon::Params::ui_design_type(params, ui) {
        new_design = Some(new);
    }
    if let Some(new) = dessins_lib::chapter_1::star::Params::ui_design_type(params, ui) {
        new_design = Some(new);
    }
    if let Some(new) = dessins_lib::chapter_1::composition_1::Params::ui_design_type(params, ui) {
        new_design = Some(new);
    }
    if let Some(new) = dessins_lib::chapter_1::composition_2::Params::ui_design_type(params, ui) {
        new_design = Some(new);
    }
    if let Some(new) = dessins_lib::chapter_1::jolygon::Params::ui_design_type(params, ui) {
        new_design = Some(new);
    }
    if let Some(new) = dessins_lib::chapter_2::Params::ui_design_type(params, ui) {
        new_design = Some(new);
    }
    if let Some(new) = dessins_lib::chapter_3::Params::ui_design_type(params, ui) {
        new_design = Some(new);
    }
    if let Some(new) = dessins_lib::chapter_4::Params::ui_design_type(params, ui) {
        new_design = Some(new);
    }
    if let Some(new) = dessins_lib::chapter_5::orbital::Params::ui_design_type(params, ui) {
        new_design = Some(new);
    }
    if let Some(new) = dessins_lib::chapter_5::rotating::Params::ui_design_type(params, ui) {
        new_design = Some(new);
    }
    if let Some(new) = dessins_lib::chapter_5::spiral::Params::ui_design_type(params, ui) {
        new_design = Some(new);
    }
    if let Some(new) = dessins_lib::chapter_6::bipartite::Params::ui_design_type(params, ui) {
        new_design = Some(new);
    }
    if let Some(new) = dessins_lib::chapter_6::linear_modulo::Params::ui_design_type(params, ui) {
        new_design = Some(new);
    }
    if let Some(new) = dessins_lib::chapter_6::linear_sticks::Params::ui_design_type(params, ui) {
        new_design = Some(new);
    }
    if let Some(new) = dessins_lib::chapter_7::Params::ui_design_type(params, ui) {
        new_design = Some(new);
    }

    new_design
}

pub fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    model.egui.handle_raw_event(event);
}

fn main() {
    nannou::app(model).update(update).run();
}
