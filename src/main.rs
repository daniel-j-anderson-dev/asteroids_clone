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
    // Ensure random number generation doesn't feel consistent to the player
    initialize_rng();

    // Define game entities
    let mut player = Player::default();
    let mut asteroids = Asteroid::many_random(10);
    let mut bullets = Bullet::many_new();

    loop {
        /* DRAW GAME */
        clear_background(BLACK);
        player.draw();
        asteroids.iter().for_each(|a| a.draw());
        bullets.iter().for_each(|b| b.draw());

        /* HANDLE INPUT */
        // Reset asteroids with space for testing
        if is_key_pressed(KeyCode::Space) {
            asteroids = Asteroid::many_random(10);
        }

        // fire a bullet with a z press
        if is_key_pressed(KeyCode::Z) {
            bullets.push(Bullet::from(&player));
        }

        /* COLLISION DETECTION */
        let mut children = Vec::new();

        for asteroid in asteroids.iter_mut() {
            for bullet in bullets.iter_mut() {
                // if the bullet is inside the asteroid
                if is_point_in_hexagon(
                    bullet.position(),
                    asteroid.position(),
                    asteroid.orientation(),
                    asteroid.size(),
                ) {
                    // calculate the children asteroids
                    let new_children = asteroid.split(bullet.velocity());

                    // collect the children
                    children.extend(new_children);

                    // destroy does NOT take ownership it just sets the is_alive field false
                    asteroid.destroy();
                    bullet.destroy();
                }
            }

            // TODO: check for collision with player and asteroid and print on all collisions
        }

        /* HANDLE COLLISION */
        // add any children from the collisions
        asteroids.append(&mut children);

        // Only keep bullets and asteroids that are alive or valid.
        bullets.retain(|b| b.is_alive() && !b.is_too_old());
        asteroids.retain(|a| a.is_alive() && !a.is_too_small());
        // Is the player alive?

        /* UPDATE GAME PHYSICS */
        player.step();
        asteroids.iter_mut().for_each(|a| a.step());
        bullets.iter_mut().for_each(|b| b.step());

        next_frame().await;
    }
}
