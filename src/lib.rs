use nannou::{color::Srgb, geom::Point2, Draw};

pub const NP: usize = 480;
pub const WEIGHT: f32 = 2.0;

pub fn draw_closed(draw: &Draw, color: Srgb<u8>, points: &[Point2]) {
    draw_exact(draw, color, &points);

    // close curve
    let last = points.last().unwrap();
    let first = points.first().unwrap();
    draw.line()
        .start(*last)
        .end(*first)
        .color(color)
        .weight(WEIGHT);
}

pub fn draw_exact(draw: &Draw, color: Srgb<u8>, points: &[Point2]) {
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
