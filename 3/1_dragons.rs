use common::{
    chapter_3::{self, DragonSettings, Model},
    NP,
};
use nannou::prelude::*;
use nannou_egui::egui::Ui;

fn model(app: &App) -> Model {
    let n = 6;

    let settings = DragonSettings {
        n,
        a0: -PI / 4.0 * (n - 2) as f32,
        l0: NP as f32 / (2.0_f32.sqrt().powf(n as f32)),
        p0: pt2(-(NP as f32) / 6.0, -(NP as f32) / 2.5),
    };

    let rules = vec![0; n as usize + 1];

    chapter_3::model(settings, &rules, app)
}

fn ui_elements(settings: &mut DragonSettings, ui: &mut Ui) -> bool {
    settings.ui_n(ui)
}

fn update(_app: &App, model: &mut Model, update: Update) {
    chapter_3::update(model, update, ui_elements)
}

fn main() {
    nannou::app(model).update(update).run();
}
