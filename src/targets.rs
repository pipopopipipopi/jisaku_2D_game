use rand::Rng;
use sdl2::rect::Rect;
use sdl2::render::{Texture, Canvas};
use sdl2::video::Window;
use crate::constants::{ FIELD_WIDTH, FIELD_HEIGHT, TILE_SIZE, TOP_MARGIN, SIDE_MARGIN };
use crate::player::Player;

pub struct Target {
    pub x: i32,
    pub y: i32,
    pub count: u32,
}

impl Target {
    pub fn new() -> Self {
        Self {
            x: 4,
            y: 0,
            count: 0,
        }
    }

    pub fn update(&mut self) {
        let mut rng = rand::rng();
        let new_x = match (self.x >= 3, self.x + 3 < FIELD_WIDTH as i32) {
            (true, true) => {
                if rng.random_bool(0.5) {
                    if self.x - 3 > 0 {
                        rng.random_range(0..self.x - 3)
                    } else {
                        0
                    }
                } else {
                    if self.x + 3 < FIELD_WIDTH as i32 {
                        rng.random_range(self.x + 3..FIELD_WIDTH as i32)
                    } else {
                        FIELD_WIDTH as i32 - 1
                    }
                }
            },
            (true, false) => {
                if self.x - 3 > 0 {
                    rng.random_range(0..self.x - 3)
                } else {
                    0
                }
            },
            (false, true) => {
                if self.x + 3 < FIELD_WIDTH as i32 {
                    rng.random_range(self.x + 3..FIELD_WIDTH as i32)
                } else {
                    FIELD_WIDTH as i32 - 1
                }
            },
            (false, false) => self.x,
        };

        let new_y = rng.random_range(0..FIELD_HEIGHT as i32);

        self.x = new_x;
        self.y = new_y;
    }
    
    pub fn draw(&self, canvas: &mut Canvas<Window>, texture: &Texture) -> Result<(), String> {
        let texture_rect = Rect::new(2 * 64, 0, 64, 64);
        let pos_rect = Rect::new(
            (self.x + SIDE_MARGIN as i32) * TILE_SIZE as i32,
            (self.y + TOP_MARGIN as i32) * TILE_SIZE as i32,
            TILE_SIZE,
            TILE_SIZE,
        );
        canvas.copy(texture, texture_rect, pos_rect)?;
        Ok(())
    }

    pub fn check_collision(&mut self, player: &Player) {
        if player.x == self.x && player.y == self.y {
            self.count += 1;
            self.update();
        }
    }
}
