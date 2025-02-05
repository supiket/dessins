use common::{self, add_float_slider, add_number_slider, Model, Segment, Shape, Shapes, NP};
use nannou::prelude::*;
use nannou_egui::egui::Ui;

struct SpiralCurveParams {
    pub n: u32, // # segments
    pub t: u32, // # times the planet turns around the sun
    pub r: f32, // flattening parameter of the ellipse
    pub l: f32, // decrease factor beween the first ellipse traveled and the last
    pub an_factor: f32,
}

fn model(app: &App) -> Model<SpiralCurveParams> {
    let curve = SpiralCurveParams {
        n: 2000,
        t: 40,
        r: 0.8,
        l: 0.1,
        an_factor: 1.0,
    };

    common::model(Box::new(SpiralCurveParams::calculate_shapes), curve, app)
}

fn update(_app: &App, model: &mut Model<SpiralCurveParams>, update: Update) {
    common::update(model, update, SpiralCurveParams::ui_elements)
}

impl SpiralCurveParams {
    fn ui_elements(&mut self, ui: &mut Ui) -> bool {
        add_number_slider(ui, "curve n", &mut self.n, 1000..=9000)
            || add_number_slider(ui, "curve t", &mut self.t, 40..=60)
            || add_float_slider(ui, "curve r", &mut self.r, 0.0..=1.0)
            || add_float_slider(ui, "curve l", &mut self.l, 0.0..=1.0)
            || add_float_slider(ui, "curve an factor", &mut self.an_factor, 1.0..=4.0)
    }

    fn calculate_shapes(&self) -> Shapes {
        let mut shapes = Shapes::new();
        let mut shape = Shape::new();
        let mut segment = Segment::new();

        let np = NP as f32;
        let n = self.n as f32;
        let t = self.t as f32;
        let r = self.r;
        let l = self.l;
        let an_factor = self.an_factor;

        for i in 0..=self.n {
            let i = i as f32;

            let rr = l.powf(i / n);
            let an = 2.0 * PI * i / n * an_factor;

            let x = rr * (t * an).cos();
            let y = rr * r * (t * an).sin();

            let co = an.cos();
            let si = an.sin();

            let xx = x * co - y * si;
            let yy = x * si + y * co;

            let x = xx * np / 2.0;
            let y = yy * np / 2.0;

            segment.push(pt2(x, y));
        }

        shape.push(segment);
        shapes.push(shape);

        shapes
    }
}

fn main() {
    nannou::app(model).update(update).run();
}
