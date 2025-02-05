use common::{self, add_float_slider_np, add_number_slider, Model, Segment, Shape, Shapes, NP};
use nannou::prelude::*;
use nannou_egui::egui::Ui;

pub struct BipartiteParams {
    pub n: u32,
    pub a: Point2,
    pub b: Point2,
    pub c: Point2,
    pub d: Point2,
}

pub type OuterSegment = Segment;
pub type InnerSegment = Segment;

pub fn update(_app: &App, model: &mut Model<BipartiteParams>, update: Update) {
    common::update(model, update, BipartiteParams::ui_elements)
}

fn model(app: &App) -> Model<BipartiteParams> {
    let params = BipartiteParams {
        n: 10,
        a: pt2((NP as f32) / -2.0, (NP as f32) / -2.0),
        b: pt2((NP as f32) / -2.0, (NP as f32) / 2.0),
        c: pt2((NP as f32) / 2.0, (NP as f32) / -2.0),
        d: pt2((NP as f32) / 2.0, (NP as f32) / 2.0),
    };

    common::model(Box::new(BipartiteParams::calculate_shapes), params, app)
}

fn main() {
    nannou::app(model).update(update).run();
}

impl BipartiteParams {
    fn calculate_points(&self) -> (OuterSegment, InnerSegment) {
        let mut outer = vec![];
        let mut inner = vec![];

        let n = self.n as f32;

        for i in 0..=self.n {
            let i = i as f32;
            let x1 = (i * self.a.x + (n - i) * self.b.x) / n;
            let y1 = (i * self.a.y + (n - i) * self.b.y) / n;
            outer.push(pt2(x1, y1));

            for j in 0..=self.n {
                let j = j as f32;

                let x2 = (j * self.c.x + (n - j) * self.d.x) / n;
                let y2 = (j * self.c.y + (n - j) * self.d.y) / n;
                inner.push(pt2(x2, y2));
            }
        }

        (outer, inner)
    }

    pub fn calculate_shapes(&self) -> Shapes {
        let mut shapes = Shapes::new();
        let mut shape = Shape::new();

        let (outer_points, inner_points) = self.calculate_points();

        for outer in &outer_points {
            for inner in &inner_points {
                let segment = vec![*outer, *inner];
                shape.push(segment);
            }
        }

        shapes.push(shape);

        shapes
    }

    pub fn ui_elements(&mut self, ui: &mut Ui) -> bool {
        let range = -0.9..=0.9;
        add_number_slider(ui, "bipartite n", &mut self.n, 10..=20)
            || add_float_slider_np(ui, "bipartite a.x", &mut self.a.x, range.clone())
            || add_float_slider_np(ui, "bipartite a.y", &mut self.a.y, range.clone())
            || add_float_slider_np(ui, "bipartite b.x", &mut self.b.x, range.clone())
            || add_float_slider_np(ui, "bipartite b.y", &mut self.b.y, range.clone())
            || add_float_slider_np(ui, "bipartite c.x", &mut self.c.x, range.clone())
            || add_float_slider_np(ui, "bipartite c.y", &mut self.c.y, range.clone())
            || add_float_slider_np(ui, "bipartite d.x", &mut self.d.x, range.clone())
            || add_float_slider_np(ui, "bipartite d.y", &mut self.d.y, range.clone())
    }
}
