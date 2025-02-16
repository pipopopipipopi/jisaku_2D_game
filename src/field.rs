use sdl2::rect::Rect;
use sdl2::render::{Texture, TextureCreator, Canvas};
use sdl2::video::Window;
use sdl2::ttf::Font;
use sdl2::pixels::Color;
use crate::constants::{
    FIELD_WIDTH,
    FIELD_HEIGHT,
    TILE_SIZE,
    TOP_MARGIN,
    SIDE_MARGIN,
    CLEAR,
};

pub fn field_draw(canvas: &mut Canvas<Window>, texture: &Texture) -> Result<(), String> {
    let field_rect = Rect::new(0, 0, 64, 64);
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
    let field_rect = Rect::new(64, 0, 64, 64);
    let pos_rect = Rect::new(
        0,
        2 * TILE_SIZE as i32,
        TILE_SIZE,
        TILE_SIZE,
    );
    canvas.copy(texture, field_rect, pos_rect)?;
    let pos_rect = Rect::new(
        ((FIELD_WIDTH + SIDE_MARGIN) * TILE_SIZE ) as i32,
        2 * TILE_SIZE as i32,
        TILE_SIZE,
        TILE_SIZE,
    );
    canvas.copy(texture, field_rect, pos_rect)?;
    Ok(())
}

pub fn beam_draw(canvas: &mut Canvas<Window>, texture: &Texture, frame_count: u32) -> Result<(), String> {
    let mut texture_type = 0;
    if frame_count < 10 {
        texture_type = 0;
    } else if frame_count < 20 {
        texture_type = 1;
    } else if frame_count < 30 {
        texture_type = 2;
    } else if frame_count < 40 {
        texture_type = 3;
    } else if frame_count < 50 {
        texture_type = 2;
    } else if frame_count < 60 {
        texture_type = 1;
    } else if frame_count < 70 {
        texture_type = 0;
    } else if frame_count < 80 {
        texture_type = 1;
    } else if frame_count < 90 {
        texture_type = 2;
    } else if frame_count < 100 {
        texture_type = 3;
    } else if frame_count < 110 {
        texture_type = 2;
    } else if frame_count < 120 {
        texture_type = 1;
    }
    let texture_rect_right = Rect::new(texture_type * 64, 2 * 64, 64, 64);
    let texture_rect_left = Rect::new(texture_type * 64 + 256, 2 * 64, 64, 64);
    let texture_rect_bg_right = Rect::new(8 * 64, 2 * 64, 64, 64);
    let texture_rect_bg_left = Rect::new(9 * 64, 2 * 64, 64, 64);
    for y in 0..FIELD_HEIGHT {
        let pos_rect_right = Rect::new(
                ((FIELD_WIDTH + 1) * TILE_SIZE) as i32,
                ((y + TOP_MARGIN) * TILE_SIZE) as i32,
                TILE_SIZE,
                TILE_SIZE,
        );
        let pos_rect_left = Rect::new(
                0,
                ((y + TOP_MARGIN) * TILE_SIZE) as i32,
                TILE_SIZE,
                TILE_SIZE,
        );
        canvas.copy(texture, texture_rect_bg_right, pos_rect_right)?;
        canvas.copy(texture, texture_rect_bg_left, pos_rect_left)?;
        canvas.copy(texture, texture_rect_right, pos_rect_right)?;
        canvas.copy(texture, texture_rect_left, pos_rect_left)?;
    }
    Ok(())
}

pub fn shockwave_draw(canvas: &mut Canvas<Window>, texture: &Texture) -> Result<(), String> {
    let shckwave_rect = Rect::new(64, 7 * 64, 64, 64);
    for x in 0..FIELD_WIDTH {
        let pos_rect = Rect::new(
            ((x + SIDE_MARGIN) * TILE_SIZE) as i32,
            2 * TILE_SIZE as i32,
            TILE_SIZE,
            TILE_SIZE,
        );
        canvas.copy(texture, shckwave_rect, pos_rect)?;
    }
    Ok(())
}

pub fn screen_draw(
    canvas: &mut Canvas<Window>,
    texture: &Texture,
    texture_creator: &TextureCreator<sdl2::video::WindowContext>,
    font: &Font,
    count: u32,
) -> Result<(), String> {
    let screen_rect = Rect::new(0, 8 * 64, 64, 2 * 64);
    let pos_rect = Rect::new(
        0,
        0,
        TILE_SIZE,
        TILE_SIZE * 2,
    );
    canvas.copy(texture, screen_rect, pos_rect)?;

    let screen_rect = Rect::new(64, 8 * 64, 64, 2 * 64);
    for x in 0..FIELD_WIDTH {
        let pos_rect = Rect::new(
            ((x + SIDE_MARGIN) * TILE_SIZE) as i32,
            0,
            TILE_SIZE,
            TILE_SIZE * 2,
        );
        canvas.copy(texture, screen_rect, pos_rect)?;
    }

    let screen_rect = Rect::new(2 * 64, 8 * 64, 64, 2 * 64);
    let pos_rect = Rect::new(
        ((FIELD_WIDTH + 1) * TILE_SIZE) as i32,
        0,
        TILE_SIZE,
        TILE_SIZE * 2,
    );
    canvas.copy(texture, screen_rect, pos_rect)?;

    let text = if count < CLEAR {
        format!("{}/10", count)
    } else {
        "GAME CLEAR".to_string()
    };

    let surface = font.render(&text)
        .blended(Color::RGB(255, 255, 255))
        .map_err(|e| e.to_string())?;
    
    let texture = texture_creator.create_texture_from_surface(&surface)
        .map_err(|e| e.to_string())?;

    let target_rect = Rect::new(
        ((FIELD_WIDTH + SIDE_MARGIN) * TILE_SIZE) as i32 / 3,
        18,
        TILE_SIZE * 4,
        TILE_SIZE * 2 - 20,
    );

    canvas.copy(&texture, None, target_rect)?;

    Ok(())
}
