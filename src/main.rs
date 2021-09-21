mod chip8;
mod opcode;

use chip8::Chip8;
use std::fs::File;
use std::io::prelude::*;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::pixels::PixelFormatEnum;
use sdl2::EventPump;

#[macro_use]
extern crate lazy_static;

fn handle_user_input(event_pump: &mut EventPump) {
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => std::process::exit(0),
            _ => (),
        }
    }
}

fn color(b: bool) -> Color {
    match b {
        true => sdl2::pixels::Color::WHITE,
        false => sdl2::pixels::Color::BLACK,
    }
}

fn read_screen_state(chip8: &Chip8, screen_state: &mut [u8; 64 * 32 * 3]) -> bool {
    let mut idx = 0;
    let mut update = false;
    for i in 0..(64 * 32) {
        let b = chip8.display[i];
        let (r, g, b) = color(b).rgb();
        if screen_state[idx] != r || screen_state[idx + 1] != g || screen_state[idx + 2] != b {
            screen_state[idx] = r;
            screen_state[idx + 1] = g;
            screen_state[idx + 2] = b;
            update = true
        }
        idx += 3;
    }
    update
}

fn main() {
    env_logger::init();

    // init sdl2
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window(
            "Chip-8 Emulator",
            (64.0 * 10.0) as u32,
            (32.0 * 10.0) as u32,
        )
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    canvas.set_scale(10.0, 10.0).unwrap();

    let creator = canvas.texture_creator();
    let mut texture = creator
        .create_texture_target(PixelFormatEnum::RGB24, 64, 32)
        .unwrap();

    let mut screen_state = [0 as u8; 64 * 32 * 3];

    let mut f = File::open("test_opcode.ch8").expect("Failed to open the file");

    let mut program: Vec<u8> = Vec::new();
    f.read_to_end(&mut program)
        .expect("failed to read the file");

    let mut chip8 = Chip8::new();
    chip8.load_program(program);
    chip8.run_with_callback(move |chip8| {
        handle_user_input(&mut event_pump);

        if read_screen_state(chip8, &mut screen_state) {
            texture.update(None, &screen_state, 64 * 3).unwrap();
            canvas.copy(&texture, None, None).unwrap();
            canvas.present();
        }

        std::thread::sleep(std::time::Duration::new(0, 100_000_000));
    });
}
