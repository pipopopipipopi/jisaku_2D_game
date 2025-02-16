use rand::Rng;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::render::{TextureCreator, Canvas};
use sdl2::video::Window;
use sdl2::ttf::Font;
use sdl2::pixels::Color;
use crate::constants::{
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
use crate::player::Player;
use crate::targets::Target;
use crate::gimmicks::beams::Beam;
use crate::gimmicks::shockwaves::{ShockwaveType, Shockwave};
use crate::gimmicks::missiles::Missile;

#[derive(PartialEq)]
pub enum Scene {
    Start,
    Playing,
    GameOver,
    GameClear,
}

pub struct SceneManager {
    pub current_scene: Scene,
    pub beams: Vec<Beam>,
    pub shockwaves: Vec<Shockwave>,
    pub missiles: Vec<Missile>,
    pub gimmick_timer: u32,
}

impl SceneManager {
    pub fn new() -> Self {
        Self {
            current_scene: Scene::Start,
            beams: vec![],
            shockwaves: vec![],
            missiles: vec![],
            gimmick_timer: 0,
        }
    }

    pub fn handle_event(&mut self, event: &Event, player: &mut Player, target: &mut Target) -> bool {
        match self.current_scene {
            Scene::Start => match event {
                Event::KeyDown { keycode: Some(Keycode::Return), .. } => {
                    self.start_game(player, target);
                },
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    return false;
                },
                _ => {}
            },
            Scene::GameClear => match event {
                Event::KeyDown { .. } => {
                    self.current_scene = Scene::Start;
                },
                _ => {}
            },
            _ => {}
        }
        true
    }

    pub fn update(&mut self, player: &mut Player, target: &mut Target) {
        if self.current_scene == Scene::Playing {
            if !player.is_alive() {
                self.current_scene = Scene::GameOver;
                self.start_game(player, target);
            } else if target.count == CLEAR {
                self.current_scene = Scene::GameClear;
            }

            if target.count > 0 {
                if self.gimmick_timer % 120 == 0 {
                    self.gimmick_timer = 0;
                    let mut rng = rand::rng();
                    let gimmick = rng.random_range(0..3);
                    match gimmick {
                        0 => {
                            for _ in 0..rng.random_range(2..=3) {
                                self.beams.push(Beam::new(rng.random_range(0..FIELD_HEIGHT) as i32));
                            }
                        },
                        1 => {
                            for _ in 0..rng.random_range(4..=8) {
                                self.missiles.push(Missile::new(
                                    rng.random_range(0..FIELD_WIDTH - 1) as i32,
                                    rng.random_range(0..FIELD_HEIGHT - 1) as i32
                                ));
                            }
                        },
                        2 => {
                            let shockwave_type = match rng.random_range(0..3) {
                                0 => ShockwaveType::Center,
                                1 => ShockwaveType::Right,
                                _ => ShockwaveType::Left,
                            };
                            self.shockwaves.push(Shockwave::new(shockwave_type));
                        },
                        _ => {}
                    }
                }
                self.gimmick_timer += 1;
            }

            for beam in &mut self.beams {
                beam.update();
                beam.check_collision(player);
            }
            self.beams.retain(|beam| beam.frame_count < BEAM_WARNING_TIME + BEAM_ACTIVE_TIME);

            for wave in &mut self.shockwaves {
                wave.update();
                wave.check_collision(player);
            }
            self.shockwaves.retain(|wave| wave.frame_count < SHOCKWAVE_WARNING_TIME + SHOCKWAVE_SPEED * FIELD_HEIGHT);

            for missile in &mut self.missiles {
                missile.update();
                missile.check_collision(player);
            }
            self.missiles.retain(|missile| missile.frame_count < MISSILE_WARNING_TIME + MISSILE_ACTIVE_TIME);
        }
    }

    pub fn start_game(&mut self, player: &mut Player, target: &mut Target) {
        *player = Player::new();
        *target = Target::new();
        self.beams.clear();
        self.shockwaves.clear();
        self.missiles.clear();
        self.current_scene = Scene::Playing;
    }

    pub fn draw_text(
        &self,
        canvas: &mut Canvas<Window>,
        texture_creator: &TextureCreator<sdl2::video::WindowContext>,
        font: &Font,
    ) -> Result<(), String> {
        let text = "Press ENTER";
        let surface = font.render(text)
            .blended(Color::RGB(255, 255, 255))
            .map_err(|e| e.to_string())?;

        let text_texture = texture_creator.create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())?;

        let text_query = text_texture.query();
        let text_width = text_query.width;
        let text_height = text_query.height;

        let text_pos_rect = Rect::new(
            (((FIELD_WIDTH + SIDE_MARGIN * 2) * TILE_SIZE) as i32 - text_width as i32) / 2,
            ((FIELD_HEIGHT + TOP_MARGIN + BOTTOM_MARGIN) as i32 * TILE_SIZE as i32 - text_height as i32) / 2,
            text_width,
            text_height,
        );

        canvas.copy(&text_texture, None, text_pos_rect)?;

        Ok(())
    }
}
