use macroquad::prelude::*;

pub const DUCK_BMP: &'static [u8] = include_bytes!("../assets/duck.bmp");
pub fn duck_texture() -> Texture2D {
    Texture2D::from_file_with_format(DUCK_BMP, None)
}

pub const ROCK_BMP: &'static [u8] = include_bytes!("../assets/rock.bmp");
pub fn rock_texture() -> Texture2D {
    Texture2D::from_file_with_format(ROCK_BMP, None)
}
