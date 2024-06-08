use asteroids::Kinematic;
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
    let foo = Kinematic {
        position: vec2(screen_width(), screen_height()) / 2.0,
        velocity: vec2(0.0, 0.0),
        acceleration: vec2(0.0, 0.0),
    };

    loop {
        clear_background(BLACK);

        draw_rectangle(foo.position.x, foo.position.y, 2.0, 2.0, WHITE);
        next_frame().await;
    }
}
