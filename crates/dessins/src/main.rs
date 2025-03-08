use bevy::prelude::*;
use bevy_egui::{EguiContexts, EguiPlugin};
use bevy_nannou::prelude::*;
use bevy_nannou::NannouPlugin;
use dessins::{params::DesignVariant, resources::Model};

fn main() {
    let window_plugin = WindowPlugin {
        primary_window: Some(Window {
            fit_canvas_to_parent: true,
            ..default()
        }),
        ..default()
    };

    let default_plugins = DefaultPlugins.set(window_plugin);

    let model = Model::new(DesignVariant::Dragon);

    App::new()
        .add_plugins((default_plugins, NannouPlugin, EguiPlugin))
        .insert_resource(model)
        .add_systems(Startup, setup)
        .add_systems(Update, (control_params, draw_dessin))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(render::NannouCamera);
}

fn control_params(mut model: ResMut<Model>, time: Res<Time<Virtual>>, egui_ctx: EguiContexts) {
    let new_design = None;

    let (changed, color) = model.control_params(egui_ctx, *time);

    if let Some(new_color) = color {
        model.color = new_color;
    }

    if let Some(new_design) = new_design {
        model.change_design(new_design);
        model.calculate_shapes();
    } else if changed || !model.initialized() {
        model.calculate_shapes();
    }
}

fn draw_dessin(draw: Single<&Draw>, model: Res<Model>) {
    draw.background().srgba(0.1, 0.1, 0.1, 0.1); // TODO: alpha does not work as I expect

    // draw.rect()
    //     .w_h(200.0, 200.0)
    //     .color(Color::linear_rgba(0.1, 0.1, 0.1, 0.001));

    model.draw_points(draw);
}

// fn update_time(mut game_time: ResMut<GameTime>, time: Res<Time<Virtual>>) {
// game_time.0 = time.elapsed_secs_f64();
// }
