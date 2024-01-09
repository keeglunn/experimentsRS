#![allow(unused_parens)]
extern crate sketches;

use sketches::midi;
use sketches::midi::twister::constants as twister_constants;

use midir::MidiInputConnection;
use nannou::{prelude::*, image::{GenericImage, Pixel}, glam::bool};
use std::{
    sync::mpsc::{channel, Receiver},
    u8, time::Duration,
};
use wmidi::MidiMessage;
use nannou::image;
use nannou::image::GenericImageView;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    image: image::DynamicImage,
    frame: image::DynamicImage, 
    texture: wgpu::Texture,
    show_frame_count: bool,
    speed: f32,
    red_w: f32,
    green_w: f32,
    blue_w: f32,
    last_update: Duration,
    _connection: Option<MidiInputConnection<()>>,
    receiver: Receiver<Vec<u8>>,
}

fn model(app: &App) -> Model {

    let assets = app.assets_path().unwrap();
    let img_path = assets
        .join("sunset.png");

    let image = image::open(img_path).unwrap();

    let _window = app
        .new_window()
        .size(image.width(), image.height()) // set window size to image dimensions
        .view(view)
        .key_pressed(key_pressed)
        .build()
        .unwrap();

    let (tx, rx) = channel();

    Model {
        frame: image.clone(),
        texture: wgpu::Texture::from_image(app, &image),
        image,

        speed: 1.0,
        last_update: Duration::new(0, 0),
        
        red_w: 1.0,
        green_w: 1.0,
        blue_w: 1.0,

        _connection: midi::init(tx),
        receiver: rx,
        show_frame_count: false,
    }
}

fn update(app: &App, model: &mut Model, update: Update) {
    update_params(model);

    if (update.since_start - model.last_update > Duration::from_millis((250.0 / model.speed).trunc() as u64)) {
//        println!("Running update_image");
        model.last_update = update.since_start;
        update_image(model);
        
        model.texture = wgpu::Texture::from_image(app, &model.frame);
    }
}

fn update_params(model: &mut Model) {
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
                            model.speed = sketches::util::speed_func(v as f32) 
                        }
                        (wmidi::Channel::Ch1, twister_constants::ONE_ZERO) => {
                            let v: u8 = velocity.into();
                            model.red_w = sketches::util::speed_func(v as f32) 
                        }
                        (wmidi::Channel::Ch1, twister_constants::ONE_ONE) => {
                            let v: u8 = velocity.into();
                            model.green_w = sketches::util::speed_func(v as f32) 
                        }
                        (wmidi::Channel::Ch1, twister_constants::ONE_TWO) => {
                            let v: u8 = velocity.into();
                            model.blue_w = sketches::util::speed_func(v as f32) 
                        }
                        _ => {}
                    }
                }
                _ => {}
            },
        };
    }
}

fn update_image(model: &mut Model) {
    let (w, h) = model.image.dimensions();
    for grid_x in 0..w {
        for grid_y in 0..h {
            let image_pixel = model.image.get_pixel(grid_x, grid_y);

            let red = model.red_w * (image_pixel[0] as f32 / 255.0);
            let green = model.green_w * (image_pixel[1] as f32 / 255.0);
            let blue = model.blue_w * (image_pixel[2] as f32 / 255.0);
            
            let rand_pick: f32 = random_range(0.0, 1.0);
            // Give each color a win chance based on their relative value. black wins if no color wins

            if (rand_pick < red) {
                model.frame.put_pixel(grid_x, grid_y, Pixel::from_channels(255, 0, 0, 0));
            } else if (rand_pick < red + green) {
                model.frame.put_pixel(grid_x, grid_y, Pixel::from_channels(0, 255, 0, 0));
            } else if (rand_pick < red + green + blue) {
                model.frame.put_pixel(grid_x, grid_y, Pixel::from_channels(0, 0, 255, 0));
            } else {
                model.frame.put_pixel(grid_x, grid_y, Pixel::from_channels(0, 0, 0, 0));
            }
        }
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let win = app.window_rect();

    draw.background().color(BLACK);

    if (model.show_frame_count) {
        draw_frame_count(&frame, &draw, &win);
    }

    draw.texture(&model.texture);

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

