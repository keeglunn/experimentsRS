#![allow(unused_parens)]

use sketches::midi;
use sketches::midi::twister::constants as twister_constants;

use midir::MidiInputConnection;
use nannou::prelude::*;
use std::{
    sync::mpsc::{channel, Receiver},
    u8,
};
use wmidi::MidiMessage;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    show_frame_count: bool,
    speeds: [f32; 4],
    rad_positions: [f32; 4],
    _connection: Option<MidiInputConnection<()>>,
    receiver: Receiver<Vec<u8>>,
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .size(1024, 1024)
        .view(view)
        .key_pressed(key_pressed)
        .build()
        .unwrap();

    let (tx, rx) = channel();

    Model {
        show_frame_count: false,

        speeds: [1.0, 1.0, 1.0, 1.0],

        rad_positions: [0.0, 0.0, 0.0, 0.0],

        _connection: midi::init(tx),
        receiver: rx,
    }
}

fn update(_app: &App, model: &mut Model, update: Update) {
    update_speeds(model);
    update_positions(model, update.since_last)
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let win = app.window_rect();
    let big_radius: f32 = win.w() / 3.0;

    draw.background().color(BLACK);

    if (model.show_frame_count) {
        draw_frame_count(&frame, &draw, &win);
    }

    for i in 0..model.rad_positions.len() {
        let x = (model.rad_positions[i]).sin() * (big_radius * ((1.0 + i as f32) * 0.25));
        let y = (model.rad_positions[i]).cos() * (big_radius * ((1.0 + i as f32) * 0.25));

        draw.ellipse()
            .x_y(x, y)
            .radius(30 as f32)
            .stroke(WHITE)
            .stroke_weight(2 as f32)
            .color(BLACK);
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

fn update_speeds(model: &mut Model) {
    for data in model.receiver.try_iter() {
        match MidiMessage::try_from(data.as_slice()) {
            Err(e) => {
                eprint!("Invalid midi message {:?}\n", e);
            }
            Ok(midi_message) => match midi_message {
                wmidi::MidiMessage::ControlChange(channel, note, velocity) => {
                    match (channel, note) {
                        (wmidi::Channel::Ch1, twister_constants::ZERO_ZERO) => {
                            let v: u8 = velocity.into();
                            model.speeds[0] = 1.0 + ((v as f32) / 127.0) * 10.0
                        }
                        (wmidi::Channel::Ch1, twister_constants::ZERO_ONE) => {
                            let v: u8 = velocity.into();
                            model.speeds[1] = 1.0 + ((v as f32) / 127.0) * 10.0
                        }
                        (wmidi::Channel::Ch1, twister_constants::ZERO_TWO) => {
                            let v: u8 = velocity.into();
                            model.speeds[2] = 1.0 + ((v as f32) / 127.0) * 10.0
                        }
                        (wmidi::Channel::Ch1, twister_constants::ZERO_THREE) => {
                            let v: u8 = velocity.into();
                            model.speeds[3] = 1.0 + ((v as f32) / 127.0) * 10.0
                        }
                        _ => {}
                    }
                }
                _ => {}
            },
        };
    }
}

const MOV_PER_SEC: f32 = 1.0;

fn update_positions(model: &mut Model, update_time: std::time::Duration) {
    for i in 0..model.rad_positions.len() {
        let update_len = (update_time.as_secs_f32() * MOV_PER_SEC);
        model.rad_positions[i] += update_len * model.speeds[i]
    }
}
