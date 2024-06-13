use asteroids::*;
use macroquad::prelude::*;

fn settings() -> Conf {
    return Conf {
        window_title: String::from("Asteroids Clone"),
        window_width: 800,
        window_height: 800,
        high_dpi: false,
        window_resizable: false,
        fullscreen: false,
        sample_count: 100,
        icon: None,
        platform: Default::default(),
    };
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

        player.step();
        asteroid.step();

        next_frame().await;
    }
}
