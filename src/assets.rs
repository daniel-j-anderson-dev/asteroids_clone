use macroquad::prelude::*;
use std::sync::OnceLock;

static DUCK_TEXTURE: OnceLock<Texture2D> = OnceLock::new();
const DUCK_PNG: &'static [u8] = include_bytes!("../assets/duck.png");
pub fn duck_texture() -> &'static Texture2D {
    DUCK_TEXTURE.get_or_init(|| Texture2D::from_file_with_format(DUCK_PNG, None))
}

const ROCK_PNG: &'static [u8] = include_bytes!("../assets/rock.png");
static ROCK_TEXTURE: OnceLock<Texture2D> = OnceLock::new();
pub fn rock_texture() -> &'static Texture2D {
    ROCK_TEXTURE.get_or_init(|| Texture2D::from_file_with_format(ROCK_PNG, None))
}
