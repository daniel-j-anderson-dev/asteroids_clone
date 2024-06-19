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
        // find all bullet-asteroid-collision indexes
        // TODO: check for asteroid-player-collision in crate::calculate_collision
        let (collision_indexes, player_collision) = calculate_collisions(&asteroids, &bullets, &player);

        // Now that we know which which bullets and asteroids have collided we can process each of them
        for (i, j) in collision_indexes.into_iter() {
            // calculate the children asteroids
            let [child1, child2] = asteroids[i].split(bullets[j].velocity());
    
            // add each child to the collection
            asteroids.push(child1);
            asteroids.push(child2);

            // then remove the asteroid and bullet from the list of active ones
            asteroids.remove(i);
            bullets.remove(j);
        }

        // Only keep bullets that have lived a certain number of frames
        bullets.retain(|b| b.is_alive());
        // TODO: remove asteroids that are too small
            
        /* UPDATE GAME PHYSICS */
        player.step();
        asteroids.iter_mut().for_each(|a| a.step());
        bullets.iter_mut().for_each(|b| b.step());

        next_frame().await;
    }
}
