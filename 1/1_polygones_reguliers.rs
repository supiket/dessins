use common::{
    chapter_1::{
        self,
        polygon::{calculate_polygon, PolygonSettings},
        Model,
    },
    Shapes, NP,
};
use nannou::prelude::*;
use nannou_egui::{self, egui::Ui};

struct Settings {
    polygon: PolygonSettings,
}

fn model(app: &App) -> Model<Settings> {
    let settings = Settings {
        polygon: PolygonSettings {
            k: 3,
            r: NP as f32 * 0.45,
            ad: 0_f32,
        },
    };

    chapter_1::model(Box::new(calculate_shapes), settings, app)
}

fn ui_elements(settings: &mut Settings, ui: &mut Ui) -> bool {
    settings.polygon.ui_elements(ui)
}

fn update(_app: &App, model: &mut Model<Settings>, update: Update) {
    chapter_1::update(model, update, ui_elements);
}

fn calculate_shapes(settings: &Settings) -> Shapes {
    let mut line = vec![];

    for i in 0..settings.polygon.k {
        let point = calculate_polygon(&settings.polygon, i);
        line.push(point);
    }

    vec![vec![line]]
}

fn main() {
    nannou::app(model).update(update).run();
}
