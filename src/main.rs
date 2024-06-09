use asteroids::{Kinematic, Player};
use macroquad::prelude::*;

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

#[macroquad::main(settings)]
async fn main() {
    let mut player = Player {
        kinematic: Kinematic {
            position: vec2(screen_width(), screen_height()) / 2.0,
            velocity: vec2(0.0, 0.0),
            acceleration: vec2(0.0, 0.0),
        },
        lives: 3,
        orientation: 0.0,
    };

    loop {
        clear_background(BLACK);

        player.draw();

        player.handle_input();

        player.kinematic.step();

        next_frame().await;
    }
}
