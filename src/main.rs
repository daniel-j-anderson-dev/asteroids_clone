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
    loop {
        clear_background(BLACK);

        next_frame().await;
    }
}
