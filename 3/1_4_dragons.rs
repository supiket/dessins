use nannou::prelude::*;
use nannou_egui::{self, egui, Egui};

fn main() {
    nannou::app(model).update(update).run();
}

struct Settings {
    np: u32,  // # elementary steps, i.e. resolution
    n: usize, // depth of recursion
    l0: f32,  // base length
    initial_angle: f32,
    color: Srgb<u8>,
}

struct Model {
    settings: Settings,
    egui: Egui,
    rules: Vec<i32>, // turning rules
    points: Vec<Point2>,
}

fn model(app: &App) -> Model {
    let window_id = app
        .new_window()
        .view(view)
        .raw_event(raw_window_event)
        .build()
        .unwrap();
    let window = app.window(window_id).unwrap();

    let rect = app.window_rect();
    app.new_window().size(rect.w() as u32, rect.h() as u32 * 2);

    let egui = Egui::from_window(&window);

    let np = 480;
    let n = 10;

    Model {
        egui,
        settings: Settings {
            initial_angle: Default::default(), // set in [`calculate_dragon_points`]
            np,
            n,
            l0: Default::default(), // set in [`calculate_dragon_points`]
            color: rgb(random(), random(), random()),
        },
        rules: Default::default(),  // set in [`calculate_dragon_points`]
        points: Default::default(), // set in [`calculate_dragon_points`]
    }
}

fn calculate_dragon_points(model: &mut Model) {
    model.settings.initial_angle = f32::PI() / 2.0;
    model.settings.l0 =
        model.settings.np as f32 / (2.0_f32.sqrt().powf(model.settings.n as f32)) * 0.9;
    let mut rules = vec![0; model.settings.n + 1];
    for i in (0..=model.settings.n).step_by(4) {
        rules[i] = 1;
    }
    model.rules = rules;

    let mut points = Vec::new();
    let np = model.settings.np as f32;

    let mut x0 = -np / 3.0;
    let mut y0 = np / 8.0;
    let mut x1 = x0;
    let mut y1 = y0;
    let mut x2 = x0;
    let mut y2 = y0;

    let mut current_angle = model.settings.initial_angle;
    points.push(pt2(x0, y0));

    let nn = (2_i32.pow(model.settings.n as u32) - 1) as i32;

    for i in 0..=nn {
        if i == 0 {
            x0 = x1;
            y0 = y1;
            x1 = x2;
            y1 = y2;
            x2 = x2 + model.settings.l0 * current_angle.cos();
            y2 = y2 + model.settings.l0 * current_angle.sin();
        } else {
            let mut ii = i;
            let mut j = 0;

            while ii % 2 == 0 {
                ii = ii / 2;
                j += 1;
            }

            let aa = (model.rules[model.settings.n - j] * 2 - 1) as f32
                * ((((ii - 1) / 2) % 2) * 2 - 1) as f32
                * PI
                / 2.0;
            current_angle += aa;

            x0 = x1;
            y0 = y1;
            x1 = x2;
            y1 = y2;
            x2 = x2 + model.settings.l0 * current_angle.cos();
            y2 = y2 + model.settings.l0 * current_angle.sin();
        }

        points.push(pt2(
            (x0 + 3.0 * x1) / 4.0,
            (y0 as f32 + 3.0 * y1 as f32) / 4.0,
        ));
        points.push(pt2(
            (x2 + 3.0 * x1) / 4.0,
            (y2 as f32 + 3.0 * y1 as f32) / 4.0,
        ));
    }

    model.points = points;
}

fn update(_app: &App, model: &mut Model, update: Update) {
    {
        model.egui.set_elapsed_time(update.since_start);
        let ctx = model.egui.begin_frame();

        egui::Window::new("settings").show(&ctx, |ui| {
            let clicked = ui.button("random color").clicked();
            if clicked {
                model.settings.color = rgb(random(), random(), random());
            }
        });
    }

    if model.points.is_empty() {
        calculate_dragon_points(model);
    }
}

fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    // let egui handle things like keyboard and mouse input.
    model.egui.handle_raw_event(event);
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);
    let point_weight = 2.0;

    // draw dragon curve
    if model.points.len() > 1 {
        for i in 0..model.points.len() - 1 {
            let start = model.points[i];
            let end = model.points[i + 1];

            draw.line()
                .start(start)
                .end(end)
                .color(model.settings.color)
                .weight(point_weight);
        }
    }

    draw.to_frame(app, &frame).unwrap();
    model.egui.draw_to_frame(&frame).unwrap();
}
