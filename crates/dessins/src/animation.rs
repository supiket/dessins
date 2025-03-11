use bevy::reflect::Reflect;
use nannou::prelude::*;

#[derive(Clone, Debug, PartialEq, Reflect)]
pub struct Animation {
    start_time: f64,
    phase_offset: f32,
}

impl Animation {
    pub fn new(time: Time<Virtual>, old_val: f32, min: f32, max: f32) -> Self {
        let t = time.elapsed_secs_f64();
        let range = max - min;

        if range == 0.0 {
            return Animation {
                start_time: t,
                phase_offset: 0.0,
            };
        }

        let old_val = if old_val < min {
            min
        } else if old_val > max {
            max
        } else {
            old_val
        };

        let normalized_old = (old_val - min) / range;
        let clamped_old = normalized_old.clamp(-1.0, 1.0);
        let mut phase_offset = clamped_old.asin();
        let mid = (min + max) / 2.0;

        if old_val > mid {
            phase_offset = std::f32::consts::PI - phase_offset;
        }

        Animation {
            start_time: t,
            phase_offset,
        }
    }

    pub fn update_value_sine(&self, time: Time<Virtual>, freq: f32, min: f32, max: f32) -> f32 {
        let dt = (time.elapsed_secs_f64() - self.start_time) as f32;
        let sine_val = (dt * freq * std::f32::consts::TAU + self.phase_offset).sin();

        min + (max - min) * (0.5 + 0.5 * sine_val)
    }
}
