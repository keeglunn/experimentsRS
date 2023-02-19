#![allow(unused_parens)]

use nannou::prelude::*;

fn main() {
    nannou::app(model).run();
}

struct Model {
    show_frame_count: bool,
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .size(1024, 1024)
        .view(view)
        .key_pressed(key_pressed)
        .build()
        .unwrap();

    Model {
        show_frame_count: false,
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let win = app.window_rect();
    let big_radius: f32 = win.w() / 3.0;

    draw.background().color(BLACK);

    if (model.show_frame_count) {
        draw_frame_count(&frame, &draw, &win);
    }

    draw.ellipse()
        .x_y(0 as f32, 0 as f32)
        .radius(big_radius)
        .stroke(WHITE)
        .stroke_weight(0.5 as f32)
        .no_fill();

    let x = (frame.nth() as f32 / 60.0).sin() * big_radius;
    let y = (frame.nth() as f32 / 60.0).cos() * big_radius;

    draw.ellipse()
        .x_y(x, y)
        .radius(30 as f32)
        .stroke(WHITE)
        .stroke_weight(2 as f32)
        .color(BLACK);

    // Write to the window frame.
    draw.to_frame(app, &frame).unwrap();
}

fn key_pressed(app: &App, model: &mut Model, key: Key) {
    if key == Key::S {
        app.main_window()
            .capture_frame(app.exe_name().unwrap() + ".png");
    }
    if key == Key::K {
        model.show_frame_count = !model.show_frame_count;
    }
}

fn draw_frame_count(frame: &Frame, draw: &Draw, win: &Rect) {
    let frame_count = frame.nth();

    draw.text(&frame_count.to_string().as_str())
        .x_y(win.right() - 15 as f32, win.top() - 15 as f32)
        .color(WHITE);
}
