use common::{
    chapter_1::{
        self,
        polygon::{calculate_polygon, PolygonSettings},
        star::{calculate_stars, StarSettings},
        Model,
    },
    Shapes, NP,
};
use nannou::prelude::*;
use nannou_egui::{self, egui::Ui};

struct Settings {
    polygon: PolygonSettings,
    star: StarSettings,
}

fn model(app: &App) -> Model<Settings> {
    let settings = Settings {
        polygon: PolygonSettings {
            k: 5,
            r: NP as f32 * 0.27,
            ad: PI / 2.0,
        },
        star: StarSettings {
            k: 25,
            h: 12,
            r: NP as f32 * 0.22,
            ad: PI / 2.0,
        },
    };

    chapter_1::model(Box::new(calculate_shapes), settings, app)
}

fn ui_elements(settings: &mut Settings, ui: &mut Ui) -> bool {
    settings.polygon.ui_elements(ui) || settings.star.ui_elements(ui)
}

fn update(_app: &App, model: &mut Model<Settings>, update: Update) {
    chapter_1::update(model, update, ui_elements);
}

fn calculate_shapes(settings: &Settings) -> Shapes {
    let mut shape = vec![];

    for i in 0..settings.polygon.k {
        let polygon_point = calculate_polygon(&settings.polygon, i);

        shape.push(vec![]);

        for j in 0..settings.star.k {
            let star_point = calculate_stars(&settings.star, j);
            let point = star_point + polygon_point;
            shape[i as usize].push(point);
        }
    }

    vec![shape]
}

fn main() {
    nannou::app(model).update(update).run();
}
