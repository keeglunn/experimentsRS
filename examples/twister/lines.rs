#![allow(unused_parens)]
extern crate sketches;

use sketches::midi;
use sketches::midi::twister::constants as twister_constants;

use midir::MidiInputConnection;
use nannou::prelude::*;
use std::{
    sync::mpsc::{channel, Receiver},
    u8,
};
use wmidi::{MidiMessage, U7};


fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    show_frame_count: bool,
    offsets: [f32; 3],
    points_mod: f32,
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
        offsets: [0.0, 0.0, 0.0],
        points_mod: 1.0,
        _connection: midi::init(tx),
        receiver: rx,
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    update_offsets(model);
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let win = app.window_rect();

    draw.background().color(BLACK);

    let p_1 = pt2(-500.0, 0.0 + model.offsets[0] * 500.0);
    let p_2 = pt2(0.0, 0.0 + model.offsets[1] * 500.0);
    let p_3 = pt2(500.0, 0.0 + model.offsets[2] * 500.0);

    let num_points = ((model.points_mod * 20.0).floor() as i32);

    for i in 0..num_points {
        let t_s = i as f32 / (num_points as f32); 
        let t_e = (i + 1) as f32 / (num_points as f32);

        let p_s = p_2 + ((1.0 - t_s).powi(2) * (p_1 - p_2)) + (t_s.powi(2) * (p_3 - p_2));
        let p_e = p_2 + ((1.0 - t_e).powi(2) * (p_1 - p_2)) + (t_e.powi(2) * (p_3 - p_2));

        draw.line()
            .start(p_s)
            .end(p_e)
            .color(WHITE)
            .stroke_weight(3.0);
    }

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

fn update_offsets(model: &mut Model) {
    for data in model.receiver.try_iter() {
        match MidiMessage::try_from(data.as_slice()) {
            Err(e) => {
                eprint!("Invalid midi message {:?}\n", e);
            }
            Ok(midi_message) => match midi_message {
                wmidi::MidiMessage::ControlChange(channel, note, velocity) => {
                    let v: f32 = (<U7 as Into<u8>>::into(velocity)) as f32;
                    match (channel, note) {
                        (wmidi::Channel::Ch1, twister_constants::ZERO_ZERO) => {
                            model.offsets[0] = 0.0 + ((v - 64.0) / 64.0)
                        }
                        (wmidi::Channel::Ch1, twister_constants::ZERO_ONE) => {
                            model.offsets[1] = 0.0 + ((v - 64.0) / 64.0)
                        }
                        (wmidi::Channel::Ch1, twister_constants::ZERO_TWO) => {
                            model.offsets[2] = 0.0 + ((v - 64.0) / 64.0)
                        }
                        (wmidi::Channel::Ch1, twister_constants::THREE_ZERO) => {
                            model.points_mod = 0.0 + (v / 127.0)
                        }
                        _ => {}
                    }
                }
                _ => {}
            },
        };
    }
}
