extern crate sdl2;

mod constants;
mod player;
mod gimmicks;
mod scene;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use constants::{
    FIELD_WIDTH,
    FIELD_HEIGHT,
    TILE_SIZE,
    TOP_MARGIN,
    BOTTOM_MARGIN,
    SIDE_MARGIN,
    BEAM_WARNING_TIME,
    BEAM_ACTIVE_TIME,
    MISSILE_WARNING_TIME,
    MISSILE_ACTIVE_TIME,
};
use player::Player;
use gimmicks::beams::Beam;
use gimmicks::shockwaves::{ShockwaveType, Shockwave};
use gimmicks::missiles::Missile;

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
    let mut beams: Vec<Beam> = vec![];
    let mut shockwaves: Vec<Shockwave> = vec![];
    let mut missiles: Vec<Missile> = vec![];
    // test
    // beams.push(Beam::new(0));
    // shockwaves.push(Shockwave::new(ShockwaveType::Left));
    // missiles.push(Missile::new(0, 0));

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

        for beam in &mut beams {
            beam.update();
            beam.check_collision(&mut player);
        }
        beams.retain(|beam| beam.frame_count < BEAM_WARNING_TIME + BEAM_ACTIVE_TIME);

        for wave in &mut shockwaves {
            wave.update();
            wave.check_collision(&mut player);
        }
        shockwaves.retain(|wave| wave.active);

        for missile in &mut missiles {
            missile.update();
            missile.check_collision(&mut player);
        }
        missiles.retain(|missile| missile.frame_count < MISSILE_WARNING_TIME + MISSILE_ACTIVE_TIME);

        if !player.is_alive() {
            println!("Game Over!");
            scene::game_over();
            break 'running;
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

        for beam in &beams {
            beam.get_color(&mut canvas);
            canvas.fill_rect(beam.get_rect())?;
        }

        for wave in &shockwaves {
            wave.get_color(&mut canvas);
            canvas.fill_rect(wave.get_rect())?;
        }

        for missile in &missiles {
            missile.get_color(&mut canvas);
            canvas.fill_rect(missile.get_rect())?;
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
