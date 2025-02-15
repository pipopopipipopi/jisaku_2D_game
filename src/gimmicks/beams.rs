use sdl2::rect::Rect;
use sdl2::render::{Texture, Canvas};
use sdl2::video::Window;
use crate::constants::{
    FIELD_WIDTH,
    TILE_SIZE,
    TOP_MARGIN,
    SIDE_MARGIN,
    BEAM_WARNING_TIME,
    BEAM_ACTIVE_TIME,
};
use crate::player::Player;

pub struct Beam {
    pub y: i32,
    pub warning: bool,
    pub active: bool,
    pub frame_count: u32,
}

impl Beam {
    pub fn new(y: i32) -> Self {
        Self {
            y,
            warning: true,
            active: false,
            frame_count: 0,
        }
    }

    pub fn update(&mut self) {
        self.frame_count += 1;

        if self.warning && self.frame_count >= BEAM_WARNING_TIME {
            self.warning = false;
            self.active = true;
        } else if self.active && self.frame_count >= BEAM_WARNING_TIME + BEAM_ACTIVE_TIME {
            self.active = false;
        }
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>, texture: &Texture) -> Result<(), String> {
        let mut texture_type = 0;
        if self.warning && self.frame_count < 10 {
            texture_type = 0;
        } else if self.warning && self.frame_count < 20 {
            texture_type = 1;
        } else if self.warning && self.frame_count < 30 {
            texture_type = 0;
        } else if self.active && self.frame_count - BEAM_WARNING_TIME < 10 {
            texture_type = 2;
        } else if self.active && self.frame_count - BEAM_WARNING_TIME < 20 {
            texture_type = 3;
        } else if self.active && self.frame_count - BEAM_WARNING_TIME < 30 {
            texture_type = 4;
        } else if self.active && self.frame_count - BEAM_WARNING_TIME < 40 {
            texture_type = 5;
        } else {
            texture_type = 6;
        }
        let texture_rect = Rect::new(texture_type * 64, 64, 64, 64);
        for x in 0..FIELD_WIDTH {
            let pos_rect = Rect::new(
                ((x + SIDE_MARGIN) * TILE_SIZE) as i32,
                (self.y + TOP_MARGIN as i32) * TILE_SIZE as i32,
                TILE_SIZE,
                TILE_SIZE,
            );
            canvas.copy(texture, texture_rect, pos_rect)?;
        }
        Ok(())
    }

    pub fn check_collision(&mut self, player: &mut Player) {
        if self.active && player.y == self.y {
            self.active = false;
            player.take_damage();
        }
    }
}
