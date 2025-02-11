use crate::constants::{ FIELD_WIDTH, FIELD_HEIGHT };

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
}
