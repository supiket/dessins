use crate::{
    add_float_slider_np_length, add_float_slider_np_position, add_float_slider_pi,
    add_number_slider,
    chapter_1::polygon::{self},
    DesignParams, Model, Segment, Shape, Shapes, NP,
};
use nannou::prelude::*;
use nannou_egui::egui::Ui;

pub type OuterSegment = Segment;
pub type InnerSegment = Segment;

pub type UiElements = Box<dyn Fn(&mut ParamsInner, &mut Ui) -> bool>;

pub struct ParamsInner {
    pub m: usize, // # of segments in starting curve
    pub n: usize, // # of sub-segments per segment
    pub k: usize, // depth
    pub positions: Vec<Point2>,
    pub lengths: Vec<f32>,
    pub angles: Vec<f32>,
}

pub struct Params {
    pub inner: ParamsInner,
    pub calculate_shapes: Box<dyn Fn(&ParamsInner) -> Shapes>,
    pub ui_elements: UiElements,
}

pub fn model(app: &App, inner: ParamsInner) -> Model {
    let params = DesignParams::SimpleFractal(Params {
        inner,
        calculate_shapes: Box::new(ParamsInner::calculate_shapes),
        ui_elements: Box::new(ParamsInner::ui_elements),
    });

    crate::model(params, app)
}

impl ParamsInner {
    pub fn ui_elements(&mut self, ui: &mut Ui) -> bool {
        let m_changed = add_number_slider(ui, "m", &mut self.m, 1..=10);
        let n_changed = add_number_slider(ui, "n", &mut self.n, 1..=10);

        if m_changed {
            self.positions = calculate_positions(self.m);
        }

        if n_changed {
            self.lengths = calculate_lengths(self.m, self.n);
            self.angles = calculate_angles(self.m, self.n);
        }

        m_changed
            || n_changed
            || add_number_slider(ui, "k", &mut self.k, 1..=10)
            || add_positions_sliders(ui, &mut self.positions)
            || add_lengths_sliders(ui, &mut self.lengths)
            || add_angles_sliders(ui, &mut self.angles)
    }

    pub fn calculate_shapes(&self) -> Shapes {
        assert_eq!(self.positions.len(), self.m + 1);
        assert_eq!(self.lengths.len(), self.n);
        assert_eq!(self.angles.len(), self.n);

        let mut shapes = Shapes::new();
        let mut shape = Shape::new();

        for ii in 0..self.m {
            let mut segment = Segment::new();

            let source = self.positions[ii];
            let destination = self.positions[ii + 1];
            let diff = destination - source;

            let mut point = source;
            segment.push(point);

            let angle = if diff.x == 0.0 {
                PI / 2.0 * if diff.y < 0.0 { -1.0 } else { 1.0 }
            } else {
                (diff.y / diff.x).atan()
            } + if diff.x < 0.0 { PI } else { 0.0 };

            let length = diff.length();

            for i in 0..(self.n).pow(self.k as u32) {
                let mut current_length = length;
                let mut current_angle = angle;
                let mut t1 = i;
                if self.k != 0 {
                    for j in (0..self.k).rev() {
                        let r = (self.n).pow(j as u32);
                        let t2 = t1 / r;
                        current_angle += self.angles[t2];
                        current_length *= self.lengths[t2];
                        t1 -= t2 * r;
                    }
                }
                point += pt2(
                    current_length * current_angle.cos(),
                    current_length * current_angle.sin(),
                );
                segment.push(point);
            }
            shape.push(segment);
        }

        shapes.push(shape);

        shapes
    }
}

impl Params {
    pub fn ui_design_type(
        current_design: &crate::DesignParams,
        ui: &mut nannou_egui::egui::Ui,
    ) -> Option<crate::DesignParams> {
        let enabled = match current_design {
            crate::DesignParams::SimpleFractal(_) => false,
            _ => true,
        };
        if ui
            .add_enabled(enabled, nannou_egui::egui::Button::new("simple fractal"))
            .clicked()
        {
            return Some(crate::DesignParams::SimpleFractal(Params::default()));
        }
        None
    }
}

impl Default for Params {
    fn default() -> Self {
        let np = NP as f32;
        Self {
            inner: ParamsInner {
                m: 4,
                n: 4,
                k: 5,
                positions: vec![
                    pt2(-0.5 * np, -0.5 * np),
                    pt2(0.5 * np, -0.5 * np),
                    pt2(0.5 * np, 0.5 * np),
                    pt2(-0.5 * np, 0.5 * np),
                    pt2(-0.5 * np, -0.5 * np),
                ],
                lengths: vec![1.0 / (2.0 + 2.0 * (0.45 * PI).cos()); 4],
                angles: vec![0.0, 0.45 * PI, -0.45 * PI, 0.0],
            },
            calculate_shapes: Box::new(ParamsInner::calculate_shapes),
            ui_elements: Box::new(ParamsInner::ui_elements),
        }
    }
}

fn add_positions_sliders(ui: &mut Ui, positions: &mut [Point2]) -> bool {
    let mut recalculate = false;
    for (i, position) in positions.iter_mut().enumerate() {
        recalculate = recalculate
            || add_float_slider_np_position(ui, &format!("positions[{}].x", i), &mut position.x)
            || add_float_slider_np_position(ui, &format!("positions[{}].y", i), &mut position.y);
    }
    recalculate
}

fn add_lengths_sliders(ui: &mut Ui, lengths: &mut [f32]) -> bool {
    let mut recalculate = false;
    for (i, length) in lengths.iter_mut().enumerate() {
        recalculate =
            recalculate || add_float_slider_np_length(ui, &format!("lengths[{}]", i), length);
    }
    recalculate
}

fn add_angles_sliders(ui: &mut Ui, angles: &mut [f32]) -> bool {
    let mut recalculate = false;
    for (i, angle) in angles.iter_mut().enumerate() {
        recalculate = recalculate || add_float_slider_pi(ui, &format!("angles[{}]", i), angle);
    }
    recalculate
}

fn calculate_positions(m: usize) -> Vec<Point2> {
    let params = polygon::ParamsInner {
        k: m as u32,
        r: NP as f32 * 0.5,
        ad: 0.0,
    };
    let mut points = vec![];
    for i in 0..m {
        let point = params.calculate_point(i as u32);
        points.push(point);
    }
    points.push(points[0]);
    points
}

fn calculate_lengths(m: usize, n: usize) -> Vec<f32> {
    vec![1.0 / (m as f32); n]
}

fn calculate_angles(m: usize, n: usize) -> Vec<f32> {
    let mut angles = vec![0.0];

    for i in 1..(n - 1) {
        angles.push((PI / (m as f32)) * if i % 2 == 1 { 1.0 } else { -1.0 });
    }

    angles.push(0.0);

    angles
}
