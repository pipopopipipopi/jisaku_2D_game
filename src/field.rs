use sdl2::rect::Rect;
use sdl2::render::{Texture, Canvas};
use sdl2::video::Window;
use crate::constants::{FIELD_WIDTH, FIELD_HEIGHT, TILE_SIZE, TOP_MARGIN, SIDE_MARGIN};

pub fn draw(canvas: &mut Canvas<Window>, texture: &Texture) -> Result<(), String> {
    let field_rect = Rect::new(0,0,64,64);
    for y in 0..FIELD_HEIGHT {
        for x in 0..FIELD_WIDTH {
            let pos_rect = Rect::new(
                ((x + SIDE_MARGIN) * TILE_SIZE) as i32,
                ((y + TOP_MARGIN) * TILE_SIZE) as i32,
                TILE_SIZE,
                TILE_SIZE,
            );
            canvas.copy(texture, field_rect, pos_rect)?;
        }
    }
    Ok(())
}
