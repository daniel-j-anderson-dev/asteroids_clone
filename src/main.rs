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
    let mut bullets: Vec<Bullet> = Vec::new();

    loop {
        clear_background(BLACK);

        if is_key_pressed(KeyCode::Space) {
            asteroid = Asteroid::random();
        }

        if is_key_pressed(KeyCode::Z) {
            // Spawn bullet
            bullets.push(Bullet::new(player.vertices()[0], player.orientation()));
        }

        

        player.draw();
        asteroid.draw();
        // for mut bullet in &mut bullets {
        //     bullet.draw()
        // }

        player.step();
        asteroid.step();
        // for mut bullet in &mut [bullets] {
        //     bullet.step()
        // }

        next_frame().await;
    }
}
