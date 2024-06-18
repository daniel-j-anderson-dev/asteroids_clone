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
    let mut asteroids = Asteroid::many_random(10);
    let mut bullets = Bullet::many_new();

    loop {
        clear_background(BLACK);

        if is_key_pressed(KeyCode::Space) {
            asteroids = Asteroid::many_random(10);
        }

        if is_key_pressed(KeyCode::Z) {
            bullets.push(Bullet::from(&player));
        }

        bullets.retain(|b| b.is_alive());

        // for every asteroid on screen
        'outer: for i in 0..asteroids.len() {
            // linear search for collisions with each bullet
            for j in 0..bullets.len() {
                // if the bullet is inside the asteroid (collision)
                if is_point_in_hexagon(
                    bullets[j].position(),
                    asteroids[i].position(),
                    asteroids[i].orientation(),
                    asteroids[i].size(),
                ) {
                    // then remove that asteroid
                    let parent = asteroids.remove(i);
                    
                    // split the parent apart
                    let (child1, child2) = parent.split(bullets[j].velocity());
                    
                    // and destroy the bullet
                    bullets.remove(j);

                    // and add each child to the collection
                    asteroids.push(child1);
                    asteroids.push(child2);

                    break 'outer;
                }
            }

            // TODO: check for collision with player and asteroids[i]
        }

        player.draw();
        asteroids.iter().for_each(|a| a.draw());
        bullets.iter().for_each(|b| b.draw());
        
        player.step();
        asteroids.iter_mut().for_each(|a| a.step());
        bullets.iter_mut().for_each(|b| b.step());

        next_frame().await;
    }
}
