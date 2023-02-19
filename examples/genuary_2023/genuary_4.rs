use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

enum Direction {
    NorthSouth,
    SouthNorth,
    EastWest,
    WestEast,
}

struct Axis {
    dir: Direction,
    position: f32,
    length: f32,
}

struct Car {
    axis: Axis,
    speed: f32,
    x: f32,
}

struct Model {
    show_frame_count: bool,
    cars: Vec<Car>,
}

fn model(app: &App) -> Model {
    let win_width = 1024;
    let win_height = 1024;

    let _window = app
        .new_window()
        .size(win_width, win_height)
        .view(view)
        .key_pressed(key_pressed)
        .build()
        .unwrap();

    Model {
        show_frame_count: false,
        cars: Vec::new(),
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let time = app.time;
    model.cars.iter_mut().enumerate().for_each(|(_i, car)| {
        car.x += car.speed * time;
    });

    model.cars.retain(|car| {
        car.x < car.axis.length
    });

    

}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let win = app.window_rect();
    
    draw.background().color(BLACK);

    if (model.show_frame_count) {
        draw_frame_count(&frame, &draw, &win);
    }

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
