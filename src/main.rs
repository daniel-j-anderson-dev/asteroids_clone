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
    const RNG_SEED: u64 = 74386193;
    rand::srand(RNG_SEED);
}

#[macroquad::main(settings)]
async fn main() {
    initialize_rng();

    let mut player = Player::default();

    loop {
        clear_background(BLACK);

        player.draw();

        player.handle_input();

        next_frame().await;
    }
}
