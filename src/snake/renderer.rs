use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use crate::snake::game::{GameState, Position};

const SNAKE_COLOR: Color = Color::RGB(255, 255, 0);
const APPLE_COLOR: Color = Color::RGB(255, 0, 0);

pub fn render(canvas: &mut WindowCanvas, game_state: &GameState) {
    draw_border(canvas);
    draw_apple(canvas, &game_state.get_apple_position());
    draw_snake(canvas, &game_state.get_snake_positions());
    canvas.present();
}

fn draw_snake(canvas: &mut WindowCanvas, snake: &Vec<Position>) {
    for cell in snake {
        draw_square_at(canvas, cell, SNAKE_COLOR);
    }
}

fn draw_apple(canvas: &mut WindowCanvas, pos: &Position) {
    draw_square_at(canvas, pos, APPLE_COLOR);
}

fn draw_border(canvas: &mut WindowCanvas) {
    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.draw_rect(Rect::new(150, 50, 500, 500))
        .expect("Cannot draw rect");
}

fn draw_square_at(canvas: &mut WindowCanvas, pos: &Position, color: Color) {
    canvas.set_draw_color(color);
    canvas.fill_rect(Rect::new(400 + pos.col * 50 + 5, 300 + pos.row * 50 + 5, 40, 40))
        .expect("Cannot draw rect");
}