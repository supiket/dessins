use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use bevy_nannou::prelude::*;
use bevy_nannou::NannouPlugin;
use dessins::{
    draw_segment, match_calculate_shapes, match_ui_elements, ui::ui_color, DesignParams, Model,
};
use nannou::prelude::random;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    fit_canvas_to_parent: true,
                    ..default()
                }),
                ..default()
            }),
            NannouPlugin,
            EguiPlugin,
        ))
        .insert_resource(Model {
            params: DesignParams::SimpleDeformedFractal(
                dessins::chapter_7::simple_deformed_fractal::Params::default(),
            ),
            points: Default::default(),
            color: Color::srgb(random(), random(), random()),
        })
        .add_systems(Startup, setup)
        .add_systems(Update, (update_egui, update_dessins))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(render::NannouCamera);
}

fn update_egui(mut model: ResMut<Model>, mut egui_ctx: EguiContexts) {
    let ctx = egui_ctx.ctx_mut();
    let mut recalculate = false;
    let mut new_design = None;

    egui::TopBottomPanel::top("designs").show(ctx, |ui| {
        ui.horizontal(|ui| {
            new_design = design_buttons(&model.params, ui);
        });
    });

    egui::SidePanel::left("params").show(ctx, |ui| {
        recalculate = match_ui_elements(&mut model.params, ui);
        if let Some(color) = ui_color(ui) {
            model.color = color;
        }
    });

    if let Some(new_design) = new_design {
        model.params = new_design;
        model.points = match_calculate_shapes(&mut model.params);
    } else if recalculate || model.points.is_empty() {
        model.points = match_calculate_shapes(&mut model.params);
    }
}

fn update_dessins(draw: Single<&Draw>, model: Res<Model>) {
    draw.background().color(BLACK);

    model.points.iter().for_each(|shape| {
        shape
            .iter()
            .for_each(|segment| draw_segment(&draw, model.color, segment))
    });
}

fn design_buttons(params: &DesignParams, ui: &mut egui::Ui) -> Option<DesignParams> {
    let mut new_design = None;
    let mut try_design = |design: Option<DesignParams>| {
        if let Some(new) = design {
            new_design = Some(new);
        }
    };

    try_design(dessins::chapter_1::polygon::Params::ui_design_type(
        params, ui,
    ));
    try_design(dessins::chapter_1::star::Params::ui_design_type(params, ui));
    try_design(dessins::chapter_1::composition_1::Params::ui_design_type(
        params, ui,
    ));
    try_design(dessins::chapter_1::composition_2::Params::ui_design_type(
        params, ui,
    ));
    try_design(dessins::chapter_1::jolygon::Params::ui_design_type(
        params, ui,
    ));
    try_design(dessins::chapter_2::Params::ui_design_type(params, ui));
    try_design(dessins::chapter_3::Params::ui_design_type(params, ui));
    try_design(dessins::chapter_4::Params::ui_design_type(params, ui));
    try_design(dessins::chapter_5::orbital::Params::ui_design_type(
        params, ui,
    ));
    try_design(dessins::chapter_5::rotating::Params::ui_design_type(
        params, ui,
    ));
    try_design(dessins::chapter_5::spiral::Params::ui_design_type(
        params, ui,
    ));
    try_design(dessins::chapter_6::bipartite::Params::ui_design_type(
        params, ui,
    ));
    try_design(dessins::chapter_6::linear_modulo::Params::ui_design_type(
        params, ui,
    ));
    try_design(dessins::chapter_6::linear_sticks::Params::ui_design_type(
        params, ui,
    ));
    try_design(dessins::chapter_7::simple_fractal::Params::ui_design_type(
        params, ui,
    ));
    try_design(dessins::chapter_7::simple_rounded_fractal::Params::ui_design_type(params, ui));
    try_design(dessins::chapter_7::simple_deformed_fractal::Params::ui_design_type(params, ui));

    new_design
}
