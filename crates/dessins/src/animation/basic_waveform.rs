use crate::{adjustable_variable::types::Context, ui::add_numeric};
use bevy::reflect::Reflect;
use nannou::prelude::*;
use std::collections::HashMap;
use wavegen::{sawtooth, sine, square, wf, PeriodicFunction, Waveform};

#[derive(Clone, Debug, PartialEq, Reflect)]
pub struct BasicWaveform {
    variant: Variant,
    frequency: f32,
    min: f32,
    max: f32,
    phase_offset: f32,
}

#[derive(Clone, Debug, PartialEq, Reflect)]
pub enum Variant {
    Sin,
    Sawtooth,
    Square,
    Triangle,
}

impl BasicWaveform {
    pub fn new(variant: Variant, start_value: f32, frequency: f32, min: f32, max: f32) -> Self {
        let range = max - min;

        if range == 0.0 {
            return Self {
                variant,
                frequency,
                min,
                max,
                phase_offset: 0.0,
            };
        }

        let start_value = start_value.clamp(min, max);
        let normalized_old = (start_value - min) / range;
        let clamped_old = normalized_old.clamp(-1.0, 1.0);
        let mut phase_offset = clamped_old.asin();
        let mid = (min + max) / 2.0;

        if start_value > mid {
            phase_offset = std::f32::consts::PI - phase_offset;
        }

        Self {
            variant,
            frequency,
            min,
            max,
            phase_offset,
        }
    }

    pub fn new_from_values(
        variant: Variant,
        start_value: f32,
        values: &HashMap<String, f32>,
    ) -> Self {
        let frequency = values.get("frequency").unwrap_or(&0.1);
        let min = values.get("min").unwrap_or(&0.0);
        let max = values.get("max").unwrap_or(&1.0);
        Self::new(variant, start_value, *frequency, *min, *max)
    }

    pub(crate) fn calculate(&self, time: Time<Virtual>, start_time: f64) -> f32 {
        let dt = (time.elapsed_secs_f64() - start_time) as f32;
        let amplitude = (self.max - self.min) / 2.0;
        let sample_rate = 1.0 / time.delta_secs();
        let waveform = match self.variant {
            Variant::Sin => wf!(
                f32,
                sample_rate,
                sine!(self.frequency, 1.0, self.phase_offset)
            ),
            Variant::Sawtooth => wf!(
                f32,
                sample_rate,
                sawtooth!(self.frequency, 1.0, self.phase_offset)
            ),
            Variant::Square => wf!(
                f32,
                sample_rate,
                square!(self.frequency, 1.0, self.phase_offset)
            ),
            Variant::Triangle => triangle_waveform(self.frequency, sample_rate, self.phase_offset),
        };
        let sample_index = (dt * sample_rate) as usize;
        let value = waveform.iter().nth(sample_index).unwrap_or(0.0);
        self.min + amplitude * (1.0 + value)
    }

    pub(crate) fn update_ui(&mut self, ui: &mut egui::Ui, values: &mut HashMap<String, f32>) {
        add_numeric(
            ui,
            &Context::new(Default::default()),
            "frequency",
            &mut self.frequency,
            0.0..=1.0,
        );
        values.insert("frequency".to_string(), self.frequency);
    }

    pub(crate) fn values_map(&self) -> HashMap<String, f32> {
        HashMap::from([
            ("frequency".to_string(), self.frequency),
            ("min".to_string(), self.min),
            ("max".to_string(), self.max),
        ])
    }

    pub(crate) fn name(&self) -> String {
        match self.variant {
            Variant::Sin => "sin".to_string(),
            Variant::Sawtooth => "sawtooth".to_string(),
            Variant::Square => "square".to_string(),
            Variant::Triangle => "triangle".to_string(),
        }
    }
}

fn triangle_waveform(frequency: f32, sample_rate: f32, phase: f32) -> Waveform<f32> {
    let mut components = Vec::new();

    for k in 0..5 {
        let harmonic_freq = (2.0 * k as f32 + 1.0) * frequency;
        let amplitude = 1.0 / ((2.0 * k as f32 + 1.0) * (2.0 * k as f32 + 1.0));

        let sin_wave: PeriodicFunction<f32> = sine!(harmonic_freq, amplitude, phase);

        components.push(sin_wave);
    }

    Waveform::with_components(sample_rate, components)
}
