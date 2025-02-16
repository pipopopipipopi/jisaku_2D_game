extern crate sdl2;

mod constants;
mod player;
mod gimmicks;
mod field;
mod targets;
mod scenes;

use rand::Rng;
use sdl2::event::Event;
use sdl2::image::{InitFlag, LoadTexture};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
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
    SHOCKWAVE_WARNING_TIME,
    SHOCKWAVE_SPEED,
    MISSILE_WARNING_TIME,
    MISSILE_ACTIVE_TIME,
    CLEAR,
};
use player::{Direction, Player};
use gimmicks::beams::Beam;
use gimmicks::shockwaves::{ShockwaveType, Shockwave};
use gimmicks::missiles::Missile;
use targets::Target;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG)?;
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
    let window_width = (FIELD_WIDTH + SIDE_MARGIN * 2) * TILE_SIZE;
    let window_height = (FIELD_HEIGHT + TOP_MARGIN + BOTTOM_MARGIN) * TILE_SIZE;
    let window = video_subsystem
        .window("jisaku_2D_game", window_width, window_height)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let texture_creator = canvas.texture_creator();

    let font = ttf_context.load_font("assets/PressStart2P.ttf", 128)?;

    let gimmicks_texture = texture_creator.load_texture("assets/gimmicks.png")?;
    let player_texture = texture_creator.load_texture("assets/player.png")?;
    
    let mut player = Player::new();
    let mut beams: Vec<Beam> = vec![];
    let mut shockwaves: Vec<Shockwave> = vec![];
    let mut missiles: Vec<Missile> = vec![];
    let mut target = Target::new();
    
    let mut rng = rand::rng();
    let mut gimmick_timer = 0;
    let mut event_pump = sdl_context.event_pump()?;
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(Keycode::W), repeat: false, .. } |
                Event::KeyDown { keycode: Some(Keycode::Up), repeat: false, .. } => {
                    player.update(0, -1, None);
                },
                Event::KeyDown { keycode: Some(Keycode::S), repeat: false, .. } |
                Event::KeyDown { keycode: Some(Keycode::Down), repeat: false, .. } => {
                    player.update(0, 1, None);
                },
                Event::KeyDown { keycode: Some(Keycode::A), repeat: false, .. } |
                Event::KeyDown { keycode: Some(Keycode::Left), repeat: false, .. } => {
                    player.update(-1, 0, Some(Direction::Left));
                },
                Event::KeyDown { keycode: Some(Keycode::D), repeat: false, .. } |
                Event::KeyDown { keycode: Some(Keycode::Right), repeat: false, .. } => {
                    player.update(1, 0, Some(Direction::Right));
                },
                _ => {}
            }
        }

        gimmick_timer += 1;
        if gimmick_timer >= 120 {
            gimmick_timer = 0;
            let gimmick = rng.random_range(0..3);
            match gimmick {
                0 => {
                    for _ in 0..rng.random_range(2..=3) {
                        beams.push(Beam::new(rng.random_range(0..FIELD_HEIGHT) as i32));
                    }
                },
                1 => {
                    for _ in 0..rng.random_range(4..=8) {
                        missiles.push(Missile::new(rng.random_range(0..FIELD_WIDTH - 1) as i32, rng.random_range(0..FIELD_HEIGHT - 1) as i32));
                    }
                },
                2 => {
                    let shockwave_type = match rng.random_range(0..3) {
                        0 => ShockwaveType::Center,
                        1 => ShockwaveType::Right,
                        _ => ShockwaveType::Left,
                    };
                    shockwaves.push(Shockwave::new(shockwave_type));
                },
                _ => {}
            }
        }

        target.check_collision(&player);
        println!("{}", target.count);

        for beam in &mut beams {
            beam.update();
            beam.check_collision(&mut player);
        }
        beams.retain(|beam| beam.frame_count < BEAM_WARNING_TIME + BEAM_ACTIVE_TIME);

        for wave in &mut shockwaves {
            wave.update();
            wave.check_collision(&mut player);
        }
        shockwaves.retain(|wave| wave.frame_count < SHOCKWAVE_WARNING_TIME + SHOCKWAVE_SPEED * FIELD_HEIGHT);

        for missile in &mut missiles {
            missile.update();
            missile.check_collision(&mut player);
        }
        missiles.retain(|missile| missile.frame_count < MISSILE_WARNING_TIME + MISSILE_ACTIVE_TIME);

        if target.count == CLEAR {
            println!("Game Clear");
            break 'running;
        }

        if !player.is_alive() {
            println!("Game Over!");
            break 'running;
        }

        canvas.set_draw_color(Color::RGB(30, 30, 30));
        canvas.clear();
        let _ = field::field_draw(&mut canvas, &gimmicks_texture);
        let _ = field::beam_draw(&mut canvas, &gimmicks_texture, gimmick_timer);
        let _ = field::shockwave_draw(&mut canvas, &gimmicks_texture);
        let _ = field::screen_draw(&mut canvas, &gimmicks_texture, &texture_creator, &font, target.count);

        let _ = target.draw(&mut canvas, &gimmicks_texture);
        let _ = player.draw(&mut canvas, &player_texture);

        for beam in &beams {
            let _ = beam.draw(&mut canvas, &gimmicks_texture);
        }

        for wave in &shockwaves {
            let _ = wave.draw(&mut canvas, &gimmicks_texture);
        }

        for missile in &missiles {
            let _ = missile.draw(&mut canvas, &gimmicks_texture);
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
