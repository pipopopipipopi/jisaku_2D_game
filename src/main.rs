extern crate sdl2;

mod constants;
mod player;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use std::time::Duration;
use constants::{
    FIELD_WIDTH,
    FIELD_HEIGHT,
    TILE_SIZE,
    TOP_MARGIN,
    BOTTOM_MARGIN,
    SIDE_MARGIN,
};
use player::Player;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window_width = (FIELD_WIDTH + SIDE_MARGIN * 2) * TILE_SIZE;
    let window_height = (FIELD_HEIGHT + TOP_MARGIN + BOTTOM_MARGIN) * TILE_SIZE;
    let window = video_subsystem
        .window("jisaku_2D_game", window_width, window_height)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    let mut player = Player::new();

    let mut event_pump = sdl_context.event_pump()?;
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(Keycode::Up), repeat: false, .. } => {
                    player.update(0, -1);
                },
                Event::KeyDown { keycode: Some(Keycode::Down), repeat: false, .. } => {
                    player.update(0, 1);
                },
                Event::KeyDown { keycode: Some(Keycode::Left), repeat: false, .. } => {
                    player.update(-1, 0);
                },
                Event::KeyDown { keycode: Some(Keycode::Right), repeat: false, .. } => {
                    player.update(1, 0);
                },
                Event::KeyDown { keycode: Some(Keycode::W), repeat: false, .. } => {
                    player.update(0, -1);
                },
                Event::KeyDown { keycode: Some(Keycode::S), repeat: false, .. } => {
                    player.update(0, 1);
                },
                Event::KeyDown { keycode: Some(Keycode::A), repeat: false, .. } => {
                    player.update(-1, 0);
                },
                Event::KeyDown { keycode: Some(Keycode::D), repeat: false, .. } => {
                    player.update(1, 0);
                },
                _ => {}
            }
        }

        canvas.set_draw_color(Color::RGB(30, 30, 30));
        canvas.clear();
        canvas.set_draw_color(Color::RGB(150, 150, 150));
        for x in 0..=FIELD_WIDTH {
            let x_pos = (x + SIDE_MARGIN) * TILE_SIZE;
            canvas.draw_line(
                (x_pos as i32, (TILE_SIZE * TOP_MARGIN) as i32),
                (x_pos as i32, (TILE_SIZE * (FIELD_HEIGHT + TOP_MARGIN)) as i32))?;
        }
        for y in 0..=FIELD_HEIGHT {
            let y_pos = (y + TOP_MARGIN) * TILE_SIZE;
            canvas.draw_line(
                ((TILE_SIZE * SIDE_MARGIN) as i32, y_pos as i32),
                ((TILE_SIZE * (FIELD_WIDTH + SIDE_MARGIN)) as i32, y_pos as i32))?;
        }

        canvas.set_draw_color(Color::RGB(0, 255, 0));
        canvas.fill_rect(player.get_rect())?;

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
