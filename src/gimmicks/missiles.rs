use sdl2::rect::Rect;
use sdl2::render::{Texture, Canvas};
use sdl2::video::Window;
use crate::constants::{
    TILE_SIZE,
    TOP_MARGIN,
    SIDE_MARGIN,
    MISSILE_WARNING_TIME,
    MISSILE_ACTIVE_TIME,
};
use crate::player::Player;

pub struct Missile {
    pub x: i32,
    pub y: i32,
    pub warning: bool,
    pub active: bool,
    pub frame_count: u32,
}

impl Missile {
    pub fn new(x: i32, y: i32) -> Self {
        Self {
            x,
            y,
            warning: true,
            active: false,
            frame_count: 0,
        }
    }

    pub fn update(&mut self) {
        self.frame_count += 1;

        if self.warning && self.frame_count >= MISSILE_WARNING_TIME {
            self.warning = false;
            self.active = true;
        } else if self.active && self.frame_count >= MISSILE_WARNING_TIME + MISSILE_ACTIVE_TIME {
            self.active = false;
        }
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>, texture: &Texture) -> Result<(), String> {
        let mut texture_type = 0;
        if self.warning {
            texture_type = 0;
        } else if self.active && self.frame_count - MISSILE_WARNING_TIME < 10 {
            texture_type = 2;
        } else if self.active && self.frame_count - MISSILE_WARNING_TIME < 20 {
            texture_type = 4;
        } else if self.active && self.frame_count - MISSILE_WARNING_TIME < 30 {
            texture_type = 6;
        } else if self.active && self.frame_count - MISSILE_WARNING_TIME < 40 {
            texture_type = 8;
        }
        let texture_rect = Rect::new(texture_type * 64, 3 * 64, 128, 128);
        let pos_rect = Rect::new(
            (self.x + SIDE_MARGIN as i32) * TILE_SIZE as i32,
            (self.y + TOP_MARGIN as i32) * TILE_SIZE as i32,
            TILE_SIZE * 2,
            TILE_SIZE * 2,
        );
        canvas.copy(texture, texture_rect, pos_rect)?;
        Ok(())
    }

    pub fn check_collision(&mut self, player: &mut Player) {
        if self.active
            && player.x >= self.x
            && player.x <= self.x + 1
            && player.y >= self.y
            && player.y <= self.y + 1
        {
            self.active = false;
            player.take_damage();
        }
    }
}
