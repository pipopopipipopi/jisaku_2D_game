use sdl2::rect::Rect;
use sdl2::render::{Texture, Canvas};
use sdl2::video::Window;
use crate::constants::{
    FIELD_HEIGHT,
    TILE_SIZE,
    TOP_MARGIN,
    BOTTOM_MARGIN,
    SIDE_MARGIN,
    SHOCKWAVE_SPEED,
};
use crate::player::Player;

pub enum ShockwaveType {
    Right,
    Left,
    Center,
}

pub struct Shockwave {
    pub x_range: (i32, i32),
    pub y: i32,
    pub active: bool,
    pub frame_count: u32,
    pub hit_flags: Vec<bool>,
}

impl Shockwave {
    pub fn new(wave_type: ShockwaveType) -> Self {
        let x_range = match wave_type {
            ShockwaveType::Right => (0, 4),
            ShockwaveType::Left => (4, 8),
            ShockwaveType::Center => (1, 7),
        };

        let width = (x_range.1 - x_range.0 + 1) as usize;
        Self {
            x_range,
            y: 0,
            active: true,
            frame_count: 0,
            hit_flags: vec![true; width],
        }
    }

    pub fn update(&mut self) {
        self.frame_count += 1;

        if self.frame_count % SHOCKWAVE_SPEED == 0 {
            self.y += 1;

            if self.y > (FIELD_HEIGHT - BOTTOM_MARGIN) as i32 {
                self.active = false;
            }
        }
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>, texture: &Texture) -> Result<(), String> {
        let texture_rect = Rect::new(0 * 64, 5 * 64, 64, 64);
        for x in self.x_range.0..=self.x_range.1 {
            let pos_rect = Rect::new(
                (x + SIDE_MARGIN as i32) * TILE_SIZE as i32,
                (self.y + TOP_MARGIN as i32) * TILE_SIZE as i32,
                TILE_SIZE,
                TILE_SIZE,
            );
            canvas.copy(texture, texture_rect, pos_rect)?;
        }
        Ok(())
    }

    pub fn check_collision(&mut self, player: &mut Player) {
        if self.y != player.y {
            return;
        }
        let index = (player.x - self.x_range.0) as usize;
        if self.x_range.0 <= player.x && player.x <= self.x_range.1 && self.hit_flags[index] {
            self.hit_flags[index] = false;
            player.take_damage();
        }
    }
}
