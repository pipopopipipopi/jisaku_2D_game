use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::pixels::Color;
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

    pub fn get_rect(&self) -> Rect {
        Rect::new(
            (self.x + SIDE_MARGIN as i32) * TILE_SIZE as i32,
            (self.y + TOP_MARGIN as i32) * TILE_SIZE as i32,
            TILE_SIZE * 2,
            TILE_SIZE * 2,
        )
    }

    pub fn get_color(&self, canvas: &mut Canvas<Window>) {
        if self.warning {
            canvas.set_draw_color(Color::RGB(0, 255, 0));
        } else if self.active {
            canvas.set_draw_color(Color::RGB(255, 255, 0));
        } else {
            return;
        }
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
