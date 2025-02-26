use bevy_egui::egui;
use nannou::prelude::*;

pub mod chapter_1;
pub mod chapter_2;
pub mod chapter_3;
pub mod chapter_4;
pub mod chapter_5;
pub mod chapter_6;
pub mod chapter_7;
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
    Shape(chapter_2::Params),
    Dragon(chapter_3::Params),
    Fractal(chapter_4::Params),
    Orbital(chapter_5::orbital::Params),
    Rotating(chapter_5::rotating::Params),
    Spiral(chapter_5::spiral::Params),
    Bipartite(chapter_6::bipartite::Params),
    LinearModulo(chapter_6::linear_modulo::Params),
    LinearSticks(chapter_6::linear_sticks::Params),
    SimpleFractal(chapter_7::simple_fractal::Params),
    SimpleRoundedFractal(chapter_7::simple_rounded_fractal::Params),
    SimpleDeformedFractal(chapter_7::simple_deformed_fractal::Params),
}

#[derive(Resource)]
pub struct Model {
    pub params: DesignParams,
    pub points: Shapes,
    pub color: Color,
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

pub fn match_ui_elements(params: &mut DesignParams, ui: &mut egui::Ui) -> bool {
    match params {
        DesignParams::Polygon(params) => (params.ui_elements)(&mut params.inner, ui),
        DesignParams::Star(params) => (params.ui_elements)(&mut params.inner, ui),
        DesignParams::Composition1(params) => (params.ui_elements)(&mut params.inner, ui),
        DesignParams::Composition2(params) => (params.ui_elements)(&mut params.inner, ui),
        DesignParams::Jolygon(params) => (params.ui_elements)(&mut params.inner, ui),
        DesignParams::Shape(params) => (params.ui_elements)(&mut params.inner, ui),
        DesignParams::Dragon(params) => (params.ui_elements)(&mut params.inner, ui),
        DesignParams::Fractal(params) => (params.ui_elements)(&mut params.inner, ui),
        DesignParams::Orbital(params) => (params.ui_elements)(&mut params.inner, ui),
        DesignParams::Rotating(params) => (params.ui_elements)(&mut params.inner, ui),
        DesignParams::Spiral(params) => (params.ui_elements)(&mut params.inner, ui),
        DesignParams::Bipartite(params) => (params.ui_elements)(&mut params.inner, ui),
        DesignParams::LinearModulo(params) => (params.ui_elements)(&mut params.inner, ui),
        DesignParams::LinearSticks(params) => (params.ui_elements)(&mut params.inner, ui),
        DesignParams::SimpleFractal(params) => (params.ui_elements)(&mut params.inner, ui),
        DesignParams::SimpleRoundedFractal(params) => (params.ui_elements)(&mut params.inner, ui),
        DesignParams::SimpleDeformedFractal(params) => (params.ui_elements)(&mut params.inner, ui),
    }
}

pub fn match_calculate_shapes(params: &mut DesignParams) -> Shapes {
    match params {
        DesignParams::Polygon(params) => (params.calculate_shapes)(&mut params.inner),
        DesignParams::Star(params) => (params.calculate_shapes)(&mut params.inner),
        DesignParams::Composition1(params) => (params.calculate_shapes)(&mut params.inner),
        DesignParams::Composition2(params) => (params.calculate_shapes)(&mut params.inner),
        DesignParams::Jolygon(params) => (params.calculate_shapes)(&mut params.inner),
        DesignParams::Shape(params) => (params.calculate_shapes)(&mut params.inner),
        DesignParams::Dragon(params) => (params.calculate_shapes)(&mut params.inner),
        DesignParams::Fractal(params) => (params.calculate_shapes)(&mut params.inner),
        DesignParams::Orbital(params) => (params.calculate_shapes)(&mut params.inner),
        DesignParams::Rotating(params) => (params.calculate_shapes)(&mut params.inner),
        DesignParams::Spiral(params) => (params.calculate_shapes)(&mut params.inner),
        DesignParams::Bipartite(params) => (params.calculate_shapes)(&mut params.inner),
        DesignParams::LinearModulo(params) => (params.calculate_shapes)(&mut params.inner),
        DesignParams::LinearSticks(params) => (params.calculate_shapes)(&mut params.inner),
        DesignParams::SimpleFractal(params) => (params.calculate_shapes)(&mut params.inner),
        DesignParams::SimpleRoundedFractal(params) => (params.calculate_shapes)(&mut params.inner),
        DesignParams::SimpleDeformedFractal(params) => (params.calculate_shapes)(&mut params.inner),
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
