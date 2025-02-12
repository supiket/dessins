use nannou::prelude::*;

pub mod chapter_1;
pub mod chapter_2;
pub mod chapter_3;
pub mod chapter_4;
pub mod chapter_5;
pub mod chapter_6;
pub mod chapter_7;
pub mod no_params;
pub mod ui;

pub const NP: usize = 480; // # elementary steps, i.e. resolution
pub const WEIGHT: f32 = 1.0; // point weight

pub type Shapes = Vec<Shape>;
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
    SimpleFractal(chapter_7::Params),
}

pub struct Model {
    pub params: DesignParams,
    pub points: Shapes,
    pub color: Color,
}

pub fn model(params: DesignParams, app: &App) -> Model {
    app.new_window().view(view).build();

    Model {
        params,
        points: Default::default(),
        color: Color::srgb(random(), random(), random()),
    }
}

pub fn view(app: &App, model: &Model) {
    let draw = app.draw();
    draw.background().color(BLACK);

    model.points.iter().for_each(|shape| {
        shape
            .iter()
            .for_each(|segment| draw_segment(&draw, model.color, segment))
    });
}

pub fn update(app: &App, model: &mut Model) {
    let mut recalculate = false;

    {
        let mut egui_ctx = app.egui();
        let ctx = egui_ctx.get_mut();

        egui::Window::new("params").show(ctx, |ui| {
            recalculate = match_ui_elements(&mut model.params, ui);

            if let Some(color) = ui::ui_color(ui) {
                model.color = color;
            }
        });
    }

    if recalculate || model.points.is_empty() {
        model.points = match_calculate_shapes(&mut model.params)
    }
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
    }
}
