use common::{
    add_float_slider, add_number_slider,
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
    n: u32,  // # stars
    rr: f32, // reduction coefficient from one star to the next & the distance between the center of the spiral and the center of successive stars
}

fn model(app: &App) -> Model<Settings> {
    let settings = Settings {
        n: 32,
        rr: 0.9,
        polygon: PolygonSettings {
            k: 8,
            r: NP as f32 * 0.36,
            ad: 0_f32,
        },
        star: StarSettings {
            k: 16,
            h: 5,
            r: NP as f32 * 0.14,
            ad: 0_f32,
        },
    };

    chapter_1::model(Box::new(calculate_shapes), settings, app)
}

fn ui_elements(settings: &mut Settings, ui: &mut Ui) -> bool {
    add_number_slider(ui, "n", &mut settings.n, 1..=100)
        || add_float_slider(ui, "rr", &mut settings.rr, 0.0..=1.0)
        || settings.polygon.ui_elements(ui)
        || settings.star.ui_elements(ui)
}

fn update(_app: &App, model: &mut Model<Settings>, update: Update) {
    chapter_1::update(model, update, ui_elements);
}

fn calculate_shapes(settings: &Settings) -> Shapes {
    let mut shape = vec![];

    for i in 0..settings.n {
        let r2 = settings.polygon.r * settings.rr.powi(i as i32);
        let r3 = settings.star.r * settings.rr.powi(i as i32);

        let mut polygon_settings = settings.polygon.clone();
        polygon_settings.r = r2;
        let polygon_point = calculate_polygon(&polygon_settings, i);

        shape.push(vec![]);

        for j in 0..settings.star.k {
            let mut star_settings = settings.star.clone();
            star_settings.r = r3;
            let star_point = calculate_stars(&star_settings, j);
            let point = star_point + polygon_point;
            shape[i as usize].push(point);
        }
    }

    vec![shape]
}

fn main() {
    nannou::app(model).update(update).run();
}
