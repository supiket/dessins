use basic_waveform::BasicWaveform;
use bevy::reflect::Reflect;
use nannou::prelude::*;
use std::collections::HashMap;

mod basic_waveform;

#[derive(Clone, Debug, PartialEq, Reflect)]
pub struct Animation {
    start_time: f64,
    variant: AnimationVariant,
    values: HashMap<String, f32>,
}

#[derive(Clone, Debug, PartialEq, Reflect)]
pub enum AnimationVariant {
    BasicWaveform(BasicWaveform),
    Perlin(PerlinNoise),
    Brownian(BrownianMotion),
    Damped(DampedOscillation),
}

#[derive(Clone, Debug, PartialEq, Reflect)]
pub struct PerlinNoise {}
#[derive(Clone, Debug, PartialEq, Reflect)]
pub struct BrownianMotion {}
#[derive(Clone, Debug, PartialEq, Reflect)]
pub struct DampedOscillation {}

impl Animation {
    pub fn new(time: Time<Virtual>, variant: AnimationVariant) -> Self {
        let start_time = time.elapsed_secs_f64();

        let values = match variant {
            AnimationVariant::BasicWaveform(ref wf) => wf.values_map(),
            _ => todo!("only basic waveform supported for now"),
        };

        Animation {
            start_time,
            variant,
            values,
        }
    }

    pub fn calculate(&self, time: Time<Virtual>) -> f32 {
        match &self.variant {
            AnimationVariant::BasicWaveform(wf) => wf.calculate(time, self.start_time),
            _ => todo!("only basic waveform supported for now"),
        }
    }

    pub fn update_ui(&mut self, ui: &mut egui::Ui, value: f32, id: &str) {
        ui.push_id(id, |ui| {
            egui::ComboBox::from_label("animation curve")
                .selected_text(self.variant.name())
                .show_ui(ui, |ui| {
                    let sin = AnimationVariant::BasicWaveform(BasicWaveform::new_from_values(
                        basic_waveform::Variant::Sin,
                        value,
                        &self.values,
                    ));
                    let sin_name = sin.name();

                    let sawtooth = AnimationVariant::BasicWaveform(BasicWaveform::new_from_values(
                        basic_waveform::Variant::Sawtooth,
                        value,
                        &self.values,
                    ));
                    let sawtooth_name = sawtooth.name();

                    let square = AnimationVariant::BasicWaveform(BasicWaveform::new_from_values(
                        basic_waveform::Variant::Square,
                        value,
                        &self.values,
                    ));
                    let square_name = square.name();

                    let triangle = AnimationVariant::BasicWaveform(BasicWaveform::new_from_values(
                        basic_waveform::Variant::Triangle,
                        value,
                        &self.values,
                    ));
                    let triangle_name = triangle.name();

                    ui.selectable_value(&mut self.variant, sin, sin_name);
                    ui.selectable_value(&mut self.variant, sawtooth, sawtooth_name);
                    ui.selectable_value(&mut self.variant, square, square_name);
                    ui.selectable_value(&mut self.variant, triangle, triangle_name);
                })
        });

        match self.variant {
            AnimationVariant::BasicWaveform(ref mut wf) => wf.update_ui(ui, &mut self.values),
            _ => todo!("only basic waveform supported for now"),
        }
    }
}

impl AnimationVariant {
    pub fn new_sin(start_value: f32, frequency: f32, min: f32, max: f32) -> Self {
        AnimationVariant::BasicWaveform(BasicWaveform::new(
            basic_waveform::Variant::Sin,
            start_value,
            frequency,
            min,
            max,
        ))
    }

    pub fn name(&self) -> String {
        match self {
            Self::BasicWaveform(wf) => wf.name(),
            _ => todo!("only basic waveform supported for now"),
        }
    }
}
