use bevy::prelude::*;
use bevy_egui::{EguiContexts, EguiPlugin};
use bevy_nannou::prelude::*;
use bevy_nannou::NannouPlugin;
use dessins::AnimationState;
use dessins::{draw_segment, match_calculate_shapes, DesignParams, Model, Shapes};
use nannou::prelude::random;

// TODO: add docstrings for design variants
// TODO: add enum Modes

fn main() {
    let window_plugin = WindowPlugin {
        primary_window: Some(Window {
            fit_canvas_to_parent: true,
            ..default()
        }),
        ..default()
    };

    #[allow(unused_mut)]
    let mut default_plugins = DefaultPlugins.set(window_plugin);

    // TODO: open tracking issue and remove
    // #[cfg(target_arch = "wasm32")]
    // {
    //     let render_plugin = bevy::render::RenderPlugin {
    //         render_creation: bevy::render::settings::WgpuSettings {
    //             backends: Some(nannou::wgpu::Backends::GL),
    //             ..default()
    //         }
    //         .into(),
    //         ..default()
    //     };
    //     default_plugins = default_plugins.set(render_plugin);
    // }

    let mut model = Model {
        params: DesignParams::Polygon(dessins::chapter_1::polygon::Params::default()),
        points: Shapes::default(),
        color: Color::srgb(random(), random(), random()),
    };

    toggle_animation_on(&mut model, 3.0, 3.0, 20.0);

    App::new()
        .add_plugins((default_plugins, NannouPlugin, EguiPlugin))
        .insert_resource(model)
        .add_systems(Startup, setup)
        .add_systems(Update, (update_egui, update_dessins, animate_params))
        .register_type::<dessins::chapter_1::polygon::Params>()
        .run();
}

// TODO: turn this into one of several options
fn animate_params(mut model: ResMut<Model>) {
    let freq = 0.1;
    let min = 3.0;
    let max = 20.0;

    let mut new_points: Option<Shapes> = Option::None;

    // TODO: match in proc macro
    match &mut model.params {
        DesignParams::Polygon(params) => {
            if let Some(struct_mut) = params
                .as_any_mut()
                .downcast_mut::<dessins::chapter_1::polygon::Params>()
            {
                if let Some(anim_state) = &struct_mut.k_animation {
                    let new_k = anim_state.update_value(freq, min, max) as u32;
                    if struct_mut.k != new_k {
                        struct_mut.k = new_k;
                        new_points = Some(struct_mut.calculate_shapes());
                        dbg!(&struct_mut.k);
                    }
                } else {
                    todo!("animation is off");
                }
            }
        }
        _ => {
            todo!("migrating");
        }
    }

    if let Some(np) = new_points {
        model.points = np;
    }
}

fn toggle_animation_on(model: &mut Model, current_val: f32, min: f32, max: f32) {
    match &mut model.params {
        DesignParams::Polygon(params) => {
            params.k_animation = Some(AnimationState::new(current_val, min, max));
        }
        _ => {
            todo!("migrating");
        }
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(render::NannouCamera);
}

fn update_egui(mut model: ResMut<Model>, mut egui_ctx: EguiContexts) {
    let _ctx = egui_ctx.ctx_mut();
    let recalculate = false;
    let new_design = None;

    // TODO: update
    // egui::TopBottomPanel::top("designs").show(ctx, |ui| {
    //     ui.horizontal(|ui| {
    //         new_design = design_buttons(&model.params, ui);
    //     });
    // });

    // egui::SidePanel::left("params").show(ctx, |ui| {
    //     recalculate = match_ui_elements(&mut model.params, ui);
    //     if let Some(color) = ui_color(ui) {
    //         model.color = color;
    //     }
    // });

    if let Some(new_design) = new_design {
        model.params = new_design;
        model.points = match_calculate_shapes(&mut model.params);
    } else if recalculate || model.points == Shapes::default() {
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
