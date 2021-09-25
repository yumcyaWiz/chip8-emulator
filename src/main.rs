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

fn reset_keyboard(chip8: &mut Chip8) {
    for value in chip8.keyboard.iter_mut() {
        *value = false;
    }
}

fn handle_user_input(chip8: &mut Chip8, event_pump: &mut EventPump) {
    reset_keyboard(chip8);

    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => std::process::exit(0),
            Event::KeyDown {
                keycode: Some(Keycode::Num1),
                ..
            } => chip8.keyboard[0x1] = true,
            Event::KeyDown {
                keycode: Some(Keycode::Num2),
                ..
            } => chip8.keyboard[0x2] = true,
            Event::KeyDown {
                keycode: Some(Keycode::Num3),
                ..
            } => chip8.keyboard[0x3] = true,
            Event::KeyDown {
                keycode: Some(Keycode::Num4),
                ..
            } => chip8.keyboard[0xC] = true,
            Event::KeyDown {
                keycode: Some(Keycode::Q),
                ..
            } => chip8.keyboard[0x4] = true,
            Event::KeyDown {
                keycode: Some(Keycode::W),
                ..
            } => chip8.keyboard[0x5] = true,
            Event::KeyDown {
                keycode: Some(Keycode::E),
                ..
            } => chip8.keyboard[0x6] = true,
            Event::KeyDown {
                keycode: Some(Keycode::R),
                ..
            } => chip8.keyboard[0xD] = true,
            Event::KeyDown {
                keycode: Some(Keycode::A),
                ..
            } => chip8.keyboard[0x7] = true,
            Event::KeyDown {
                keycode: Some(Keycode::S),
                ..
            } => chip8.keyboard[0x8] = true,
            Event::KeyDown {
                keycode: Some(Keycode::D),
                ..
            } => chip8.keyboard[0x9] = true,
            Event::KeyDown {
                keycode: Some(Keycode::F),
                ..
            } => chip8.keyboard[0xE] = true,
            Event::KeyDown {
                keycode: Some(Keycode::Z),
                ..
            } => chip8.keyboard[0xA] = true,
            Event::KeyDown {
                keycode: Some(Keycode::X),
                ..
            } => chip8.keyboard[0x0] = true,
            Event::KeyDown {
                keycode: Some(Keycode::C),
                ..
            } => chip8.keyboard[0xB] = true,
            Event::KeyDown {
                keycode: Some(Keycode::V),
                ..
            } => chip8.keyboard[0xF] = true,
            _ => (),
        }
    }
}

fn color(b: bool) -> Color {
    match b {
        true => sdl2::pixels::Color::RGB(0, 200, 0),
        false => sdl2::pixels::Color::BLACK,
    }
}

// read screen from chip8, coloring, if pixel changed, update screen state
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

    let mut f = File::open("delay_timer_test.ch8").expect("Failed to open the file");

    let mut program: Vec<u8> = Vec::new();
    f.read_to_end(&mut program)
        .expect("failed to read the file");

    let mut chip8 = Chip8::new();
    chip8.load_program(program);
    chip8.run_with_callback(move |chip8| {
        handle_user_input(chip8, &mut event_pump);

        if read_screen_state(chip8, &mut screen_state) {
            texture.update(None, &screen_state, 64 * 3).unwrap();
            canvas.copy(&texture, None, None).unwrap();
            canvas.present();
        }
    });
}
