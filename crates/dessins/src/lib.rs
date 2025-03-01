use nannou::prelude::*;
use std::time::Instant;

pub mod chapter_1;
pub mod ui;

pub const NP: usize = 480; // # elementary steps, i.e. resolution
pub const WEIGHT: f32 = 1.0; // point weight

#[derive(PartialEq)]
pub struct Shapes(Vec<Shape>);
pub type Shape = Vec<Segment>;
pub type Segment = Vec<Point2>;

pub enum DesignParams {
    Polygon(chapter_1::polygon::Params),
    Star(chapter_1::star::Params),
    Composition1(chapter_1::composition_1::Params),
    Composition2(chapter_1::composition_2::Params),
    Jolygon(chapter_1::jolygon::Params),
}

#[derive(Resource)]
pub struct Model {
    pub params: DesignParams,
    pub points: Shapes,
    pub color: Color,
}

// TODO: extract to module
#[derive(Clone, Debug)]
pub struct AnimationState {
    start_time: Instant,
    phase_offset: f32,
}

pub fn draw_segment(draw: &Draw, color: Color, points: &[Point2]) {
    if points.len() < 2 {
        return;
    }

    for i in 0..points.len() - 1 {
        let start = points[i];
        let end = points[i + 1];

        draw.line()
            .start(start)
            .end(end)
            .color(color)
            .weight(WEIGHT);
    }
}

pub fn match_calculate_shapes(params: &mut DesignParams) -> Shapes {
    match params {
        DesignParams::Polygon(params) => params.calculate_shapes(),
        DesignParams::Star(params) => params.calculate_shapes(),
        DesignParams::Composition1(params) => params.calculate_shapes(),
        DesignParams::Composition2(params) => params.calculate_shapes(),
        DesignParams::Jolygon(params) => params.calculate_shapes(),
    }
}

impl AnimationState {
    pub fn new(old_val: f32, min: f32, max: f32) -> Self {
        let t = Instant::now();

        let normalized_old = (old_val - min) / (max - min);
        let sin_input = 2.0 * normalized_old - 1.0;
        let mut phase_offset = sin_input.asin();
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

impl Default for Shapes {
    fn default() -> Self {
        Self(vec![vec![vec![Point2::default()]]])
    }
}

impl core::ops::Deref for Shapes {
    type Target = Vec<Shape>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl core::ops::DerefMut for Shapes {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
