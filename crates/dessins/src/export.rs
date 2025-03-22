use crate::shapes::Shapes;
use nannou::prelude::*;
use svg::node::element::{path::Data, Path, SVG};

pub fn add_export_svg_button(ui: &mut egui::Ui, points: &Shapes) {
    if ui.button("export svg").clicked() {
        export_svg(points);
    }
}

pub fn export_svg(points: &Shapes) {
    let filename = format!("{}.svg", uuid::Uuid::new_v4());

    let mut document = SVG::new()
        // .set("viewBox", "-720 -720 720 720")
        // .set("viewBox", "0 0 720 720")
        .set("viewBox", "-360 -360 720 720")
        .set("width", "2000px")
        .set("height", "2000px");

    for shape in points.iter() {
        for segment in shape {
            let mut data = Data::new();
            if let Some(first) = segment.first() {
                data = data.move_to((first.x, -first.y));

                for point in &segment[1..] {
                    data = data.line_to((point.x, -point.y));
                }
            }

            let path = Path::new()
                .set("fill", "none")
                .set("stroke", "black")
                .set("stroke-width", 2)
                .set("d", data);

            document = document.add(path);
        }
    }

    svg::save(filename.to_string(), &document).unwrap();
}
