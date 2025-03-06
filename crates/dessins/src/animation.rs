use bevy::reflect::Reflect;
use std::time::Instant;

#[derive(Clone, Debug, PartialEq, Reflect)]
pub struct AnimationState {
    start_time: Instant,
    phase_offset: f32,
}

impl AnimationState {
    pub fn new(old_val: f32, min: f32, max: f32) -> Self {
        let t = Instant::now();

        let normalized_old = (old_val - min) / (max - min);
        let mut phase_offset = normalized_old.asin();
        let mid = (min + max) / 2.0;

        if old_val > mid {
            phase_offset = std::f32::consts::PI - phase_offset;
        }

        AnimationState {
            start_time: t,
            phase_offset,
        }
    }

    pub fn update_value(&self, freq: f32, min: f32, max: f32) -> f32 {
        let dt = self.start_time.elapsed().as_secs_f32();
        let sine_val = (dt * freq * std::f32::consts::TAU + self.phase_offset).sin();
        let new_val = min + (max - min) * (0.5 + 0.5 * sine_val);
        new_val.round()
    }
}
