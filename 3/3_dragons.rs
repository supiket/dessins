use common::{
    chapter_3::{self, DragonSettings, Model},
    NP,
};
use nannou::prelude::*;
use nannou_egui::egui::Ui;

fn model(app: &App) -> Model {
    let n = 10;

    let settings = DragonSettings {
        n,
        l0: NP as f32 / (2.0_f32.sqrt().powf(n as f32)) * 0.9,
        a0: -1.5 * PI,
        p0: pt2(-(NP as f32) * 0.25, (NP as f32) / 6.0),
    };

    let mut rules = vec![0; n as usize + 1];
    for i in (0..=n as usize).step_by(4) {
        rules[i] = 1;
    }

    chapter_3::model(settings, &rules, app)
}

fn ui_elements(settings: &mut DragonSettings, ui: &mut Ui) -> bool {
    settings.ui_n(ui) || settings.ui_a0(ui)
}

fn update(_app: &App, model: &mut Model, update: Update) {
    chapter_3::update(model, update, ui_elements)
}

fn main() {
    nannou::app(model).update(update).run();
}
