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
        clear_background(BLACK);

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
        // for every asteroid on screen
        let mut i = 0;
        'outer: while i < asteroids.len() {
            // TODO: check for collision with player and asteroids[i] and print on all collisions

            // linear search for asteroid-bullet collisions 
            let mut j = 0;
            while j <bullets.len() {
                // if the bullet is inside the asteroid
                if is_point_in_hexagon(
                    bullets[j].position(),
                    asteroids[i].position(),
                    asteroids[i].orientation(),
                    asteroids[i].size(),
                ) {
                    // then remove that asteroid from the list of active asteroids
                    let parent = asteroids.remove(i);
                    
                    // split the parent apart
                    // The `parent` has already been removed; regardless calling `split` here
                    // takes ownership thus dropping the `parent` value.
                    let (child1, child2) = parent.split(bullets[j].velocity());
                    
                    // and destroy the bullet
                    bullets.remove(j);

                    // and add each child to the collection
                    asteroids.push(child1);
                    asteroids.push(child2);

                    // start the search over no that the active Asteroids have changed
                    i = 0;
                    continue 'outer;
                }

                j += 1;
            }

            i += 1;
        }

        /* DRAW GAME */
        player.draw();
        asteroids.iter().for_each(|a| a.draw());
        bullets.iter().for_each(|b| b.draw());
        
        /* UPDATE GAME STATE */
        player.step();
        asteroids.iter_mut().for_each(|a| a.step());
        bullets.iter_mut().for_each(|b| b.step());
        
        bullets.retain(|b| b.is_alive());
        // TODO: remove asteroids that are too small
        asteroids.retain(|a| !a.is_too_small());

        next_frame().await;
    }
}
