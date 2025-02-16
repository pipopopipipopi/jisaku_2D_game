use sdl2::rect::Rect;
use sdl2::render::{Texture, Canvas};
use sdl2::video::Window;
use crate::constants::{ FIELD_WIDTH, FIELD_HEIGHT, TILE_SIZE, TOP_MARGIN, SIDE_MARGIN };

pub enum Direction {
    Right,
    Left,
}

pub struct Player {
    pub x: i32,
    pub y: i32,
    pub hp: u32,
    pub direction: Direction,
    pub frame_count: u32,
    pub animation_frames: [i32; 8],
    pub animation_index: usize,
}

impl Player {
    pub fn new() -> Self {
        Self {
            x: 4,
            y: 3,
            hp: 1,
            direction: Direction::Left,
            frame_count: 0,
            animation_frames: [0, 1, 2, 3, 4, 3, 2, 1],
            animation_index: 0,
        }
    }

    pub fn update(&mut self, dx: i32, dy: i32, dir: Option<Direction>) {
        let new_x = self.x + dx;
        let new_y = self.y + dy;

        if new_x >= 0 && new_x < FIELD_WIDTH as i32 && new_y >= 0 && new_y < FIELD_HEIGHT as i32 {
            self.x = new_x;
            self.y = new_y;
            if let Some(d) = dir {
                self.direction = d;
            }
        }
    }

    pub fn draw(&mut self, canvas: &mut Canvas<Window>, texture: &Texture) -> Result<(), String> {
        let mut texture_type = match self.direction {
            Direction::Left => (0, 0),
            Direction::Right => (0, 1),
        };

        self.frame_count += 1;
        if self.frame_count >= 10 {
            self.frame_count = 0;
            self.animation_index = (self.animation_index + 1) % self.animation_frames.len();
        }
        texture_type.0 = self.animation_frames[self.animation_index];

        let texture_rect = Rect::new(texture_type.0 * 64, texture_type.1 * 64, 64, 64);
        let pos_rect = Rect::new(
            (self.x + SIDE_MARGIN as i32) * TILE_SIZE as i32,
            ((self.y + TOP_MARGIN as i32) * TILE_SIZE as i32) - TILE_SIZE as i32 / 4,
            TILE_SIZE,
            TILE_SIZE,
        );
        canvas.copy(texture, texture_rect, pos_rect)?;
        Ok(())
    }

    pub fn take_damage(&mut self) {
        if self.hp > 0 {
            self.hp -= 1;
        }
    }

    pub fn is_alive(&self) -> bool {
        self.hp > 0
    }
}
