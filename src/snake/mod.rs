mod game;
mod renderer;

use std::error::Error;
use std::time::Duration;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use crate::snake::game::{Direction, GameState};

pub fn launch() -> Result<(), Box<dyn Error>> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem.window("Snake in Rust", 800, 600)
        .position_centered()
        .build()?;

    let mut canvas = window.into_canvas().build()?;
    let mut event_pump = sdl_context.event_pump()?;
    let mut game = GameState::new();

    canvas.set_draw_color(Color::RGB(1, 1, 1));
    canvas.clear();
    canvas.present();

    'running: loop {
        canvas.set_draw_color(Color::RGB(1, 1, 1));
        canvas.clear();

        let mut new_direction: Option<Direction> = None;
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
                    new_direction = Some(Direction::Up);
                },
                Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                    new_direction = Some(Direction::Down);
                },
                Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
                    new_direction = Some(Direction::Left);
                },
                Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                    new_direction = Some(Direction::Right);
                },
                _ => {}
            }
        }

        if let Some(direction) = new_direction {
            game.set_direction(direction);
        }

        game.tick();
        renderer::render(&mut canvas, &game);

        if game.is_end() {
            break 'running
        }

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 4));
    }
    'end: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'end
                },
                _ => {}
            }
        }
    }

    Ok(())
}
