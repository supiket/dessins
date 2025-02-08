use crate::{
    add_float_slider_np_length, add_float_slider_np_position, add_float_slider_pi,
    add_number_slider,
    chapter_1::polygon::{self, calculate_point},
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
        calculate_shapes: Box::new(calculate_shapes),
        ui_elements: Box::new(ui_elements),
    });

    crate::model(params, app)
}

pub fn ui_elements(inner: &mut ParamsInner, ui: &mut Ui) -> bool {
    let m_changed = add_number_slider(ui, "m", &mut inner.m, 1..=10);
    let n_changed = add_number_slider(ui, "n", &mut inner.n, 1..=10);

    if m_changed {
        inner.positions = calculate_positions(inner.m);
    }

    if n_changed {
        inner.lengths = calculate_lengths(inner.m, inner.n);
        inner.angles = calculate_angles(inner.m, inner.n);
    }

    m_changed
        || n_changed
        || add_number_slider(ui, "k", &mut inner.k, 1..=10)
        || add_positions_sliders(ui, &mut inner.positions)
        || add_lengths_sliders(ui, &mut inner.lengths)
        || add_angles_sliders(ui, &mut inner.angles)
}

pub fn calculate_shapes(inner: &ParamsInner) -> Shapes {
    assert_eq!(inner.positions.len(), inner.m + 1);
    assert_eq!(inner.lengths.len(), inner.n);
    assert_eq!(inner.angles.len(), inner.n);

    let mut shapes = Shapes::new();
    let mut shape = Shape::new();

    for ii in 0..inner.m {
        let mut segment = Segment::new();

        let source = inner.positions[ii];
        let destination = inner.positions[ii + 1];
        let diff = destination - source;

        let mut point = source;
        segment.push(point);

        let angle = if diff.x == 0.0 {
            PI / 2.0 * if diff.y < 0.0 { -1.0 } else { 1.0 }
        } else {
            (diff.y / diff.x).atan()
        } + if diff.x < 0.0 { PI } else { 0.0 };

        let length = diff.length();

        for i in 0..(inner.n).pow(inner.k as u32) {
            let mut current_length = length;
            let mut current_angle = angle;
            let mut t1 = i;
            if inner.k != 0 {
                for j in (0..inner.k).rev() {
                    let r = (inner.n).pow(j as u32);
                    let t2 = t1 / r;
                    current_angle += inner.angles[t2];
                    current_length *= inner.lengths[t2];
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
        let point = calculate_point(&params, i as u32);
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
