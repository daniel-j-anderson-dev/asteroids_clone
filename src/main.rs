use asteroids::*;
use macroquad::{prelude::*, rand};

fn settings() -> Conf {
    return Conf {
        window_title: String::from("Asteroids Clone"),
        window_width: 800,
        window_height: 800,
        high_dpi: true,
        window_resizable: false,
        fullscreen: false,
        ..Default::default()
    };
}

fn initialize_rng() {
    let current_time = macroquad::miniquad::date::now() as u64;
    rand::srand(current_time);
}

#[macroquad::main(settings)]
async fn main() {
    initialize_rng();

    let mut player = Player::default();
    let mut asteroid = Asteroid::random();

    loop {
        clear_background(BLACK);

        if is_key_pressed(KeyCode::Space) {
            asteroid = Asteroid::random();
        }

        player.draw();
        asteroid.draw();

        player.handle_input();

        next_frame().await;
    }
}
