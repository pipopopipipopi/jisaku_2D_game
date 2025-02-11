use sdl2::rect::Rect;
use crate::constants::{ FIELD_WIDTH, FIELD_HEIGHT, TILE_SIZE, TOP_MARGIN, SIDE_MARGIN };

pub struct Player {
    pub x: i32,
    pub y: i32,
}

impl Player {
    pub fn new() -> Self {
        Self {
            x: 4,
            y: 3,
        }
    }

    pub fn update(&mut self, dx: i32, dy: i32) {
        let new_x = self.x + dx;
        let new_y = self.y + dy;

        if new_x >= 0 && new_x < FIELD_WIDTH as i32 && new_y >= 0 && new_y < FIELD_HEIGHT as i32 {
            self.x = new_x;
            self.y = new_y;
        }
    }

    pub fn get_rect(&self) -> Rect {
        Rect::new(
            (self.x + SIDE_MARGIN as i32) * TILE_SIZE as i32,
            (self.y + TOP_MARGIN as i32) * TILE_SIZE as i32,
            TILE_SIZE,
            TILE_SIZE,
        )
    }
}
