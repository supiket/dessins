use dessins::{
    draw_segment, match_calculate_shapes, match_ui_elements, ui::ui_color, DesignParams, Model,
};
use nannou::prelude::*;

pub fn model(app: &App) -> Model {
    if cfg!(target_arch = "wasm32") {
        let camera = app.new_camera().build();
        let _window = app
            .new_window()
            .camera(camera)
            .primary()
            .view(dessins::view)
            .build();
    } else {
        app.new_window().view(dessins::view).build();
    }

    let params = DesignParams::SimpleDeformedFractal(
        dessins::chapter_7::simple_deformed_fractal::Params::default(),
    );

    {
        Model {
            params,
            points: Default::default(),
            color: Color::srgb(random(), random(), random()),
        }
    }
}

pub fn view(app: &App, model: &Model, _window: Entity) {
    let draw = app.draw();
    draw.background().color(BLACK);

    model.points.iter().for_each(|shape| {
        shape
            .iter()
            .for_each(|segment| draw_segment(&draw, model.color, segment))
    });
}

pub fn update(app: &App, model: &mut Model) {
    let mut recalculate = false;
    let mut new_design = Option::None;

    {
        let mut egui_ctx = app.egui();
        let ctx = egui_ctx.get_mut();

        egui::TopBottomPanel::top("designs").show(ctx, |ui| {
            ui.horizontal(|ui| {
                new_design = design_buttons(&model.params, ui);
            })
        });

        egui::SidePanel::left("params").show(ctx, |ui| {
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

fn design_buttons(params: &DesignParams, ui: &mut crate::egui::Ui) -> Option<DesignParams> {
    let mut new_design = Option::None;
    if let Some(new) = dessins::chapter_1::polygon::Params::ui_design_type(params, ui) {
        new_design = Some(new);
    }
    if let Some(new) = dessins::chapter_1::star::Params::ui_design_type(params, ui) {
        new_design = Some(new);
    }
    if let Some(new) = dessins::chapter_1::composition_1::Params::ui_design_type(params, ui) {
        new_design = Some(new);
    }
    if let Some(new) = dessins::chapter_1::composition_2::Params::ui_design_type(params, ui) {
        new_design = Some(new);
    }
    if let Some(new) = dessins::chapter_1::jolygon::Params::ui_design_type(params, ui) {
        new_design = Some(new);
    }
    if let Some(new) = dessins::chapter_2::Params::ui_design_type(params, ui) {
        new_design = Some(new);
    }
    if let Some(new) = dessins::chapter_3::Params::ui_design_type(params, ui) {
        new_design = Some(new);
    }
    if let Some(new) = dessins::chapter_4::Params::ui_design_type(params, ui) {
        new_design = Some(new);
    }
    if let Some(new) = dessins::chapter_5::orbital::Params::ui_design_type(params, ui) {
        new_design = Some(new);
    }
    if let Some(new) = dessins::chapter_5::rotating::Params::ui_design_type(params, ui) {
        new_design = Some(new);
    }
    if let Some(new) = dessins::chapter_5::spiral::Params::ui_design_type(params, ui) {
        new_design = Some(new);
    }
    if let Some(new) = dessins::chapter_6::bipartite::Params::ui_design_type(params, ui) {
        new_design = Some(new);
    }
    if let Some(new) = dessins::chapter_6::linear_modulo::Params::ui_design_type(params, ui) {
        new_design = Some(new);
    }
    if let Some(new) = dessins::chapter_6::linear_sticks::Params::ui_design_type(params, ui) {
        new_design = Some(new);
    }
    if let Some(new) = dessins::chapter_7::simple_fractal::Params::ui_design_type(params, ui) {
        new_design = Some(new);
    }
    if let Some(new) =
        dessins::chapter_7::simple_rounded_fractal::Params::ui_design_type(params, ui)
    {
        new_design = Some(new);
    }

    new_design
}

fn main() {
    if cfg!(target_arch = "wasm32") {
        nannou::app(model)
            .update(update)
            .view(view)
            .fullscreen()
            .run();
    } else {
        nannou::app(model).update(update).fullscreen().run();
    }
}
