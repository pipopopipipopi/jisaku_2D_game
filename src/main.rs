extern crate sdl2;

mod constants;
mod player;
mod gimmicks;
mod field;
mod targets;
mod scenes;

use sdl2::event::Event;
use sdl2::image::{InitFlag, LoadTexture};
use sdl2::keyboard::Keycode;
use std::time::Duration;
use constants::{
    FIELD_WIDTH,
    FIELD_HEIGHT,
    TILE_SIZE,
    TOP_MARGIN,
    BOTTOM_MARGIN,
    SIDE_MARGIN,
};
use player::{Direction, Player};
use targets::Target;
use scenes::{Scene, SceneManager};

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

    let font = ttf_context.load_font("assets/PressStart2P.ttf", 64)?;

    let gimmicks_texture = texture_creator.load_texture("assets/gimmicks.png")?;
    let player_texture = texture_creator.load_texture("assets/player.png")?;
    
    let mut target = Target::new();
    let mut player = Player::new();
    
    let mut scene_manager = SceneManager::new();
    let mut event_pump = sdl_context.event_pump()?;
    'running: loop {
        for event in event_pump.poll_iter() {
            if !scene_manager.handle_event(&event, &mut player, &mut target) {
                break 'running;
            }
            if scene_manager.current_scene == Scene::Playing {
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
        }

        scene_manager.update(&mut player, &mut target);

        canvas.clear();

        if scene_manager.current_scene == Scene::Playing {
            target.check_collision(&player);

            let _ = field::field_draw(&mut canvas, &gimmicks_texture)?;
            let _ = field::beam_draw(&mut canvas, &gimmicks_texture, scene_manager.gimmick_timer)?;
            let _ = field::shockwave_draw(&mut canvas, &gimmicks_texture)?;
            let _ = field::screen_draw(&mut canvas, &gimmicks_texture, &texture_creator, &font, target.count)?;

            let _ = target.draw(&mut canvas, &gimmicks_texture)?;
            let _ = player.draw(&mut canvas, &player_texture)?;

            for beam in &scene_manager.beams {
                let _ = beam.draw(&mut canvas, &gimmicks_texture)?;
            }
            for wave in &scene_manager.shockwaves {
                let _ = wave.draw(&mut canvas, &gimmicks_texture)?;
            }
            for missile in &scene_manager.missiles {
                let _ = missile.draw(&mut canvas, &gimmicks_texture)?;
            }
        } else if scene_manager.current_scene == Scene::GameClear {
            let _ = field::field_draw(&mut canvas, &gimmicks_texture)?;
            let _ = field::beam_draw(&mut canvas, &gimmicks_texture, scene_manager.gimmick_timer)?;
            let _ = field::shockwave_draw(&mut canvas, &gimmicks_texture)?;
            let _ = field::screen_draw(&mut canvas, &gimmicks_texture, &texture_creator, &font, target.count)?;

            let _ = target.draw(&mut canvas, &gimmicks_texture)?;
            let _ = player.draw(&mut canvas, &player_texture)?;

            for beam in &scene_manager.beams {
                let _ = beam.draw(&mut canvas, &gimmicks_texture)?;
            }
            for wave in &scene_manager.shockwaves {
                let _ = wave.draw(&mut canvas, &gimmicks_texture)?;
            }
            for missile in &scene_manager.missiles {
                let _ = missile.draw(&mut canvas, &gimmicks_texture)?;
            }
        } else {
            let _ = scene_manager.draw_text(&mut canvas, &texture_creator, &font);
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
