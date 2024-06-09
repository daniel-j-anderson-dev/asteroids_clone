use super::*;
use macroquad::prelude::*;

pub struct Player {
    pub kinematic: Kinematic,
    pub lives: usize,
    pub orientation: f32,
}
impl Player {
    pub const SIZE: f32 = 20.0;
    pub const ROTATION_DELTA: f32 = 0.1;

    const MAX_SPEED: f32 = Self::SIZE * 2.0;
    const MAX_SPEED_SQUARED: f32 = Self::MAX_SPEED * Self::MAX_SPEED;

    const POSITION_OFFSET: Vec2 = vec2(0.0, Self::SIZE / -4.0);
    const FRONT_OFFSET: Vec2 = vec2(0.0, Self::SIZE);
    const LEFT_OFFSET: Vec2 = vec2(-Self::SIZE / 2.5, 0.0);
    const RIGHT_OFFSET: Vec2 = vec2(Self::SIZE / 2.5, 0.0);
}
impl Player {
    pub fn default() -> Player {
        return Player {
            kinematic: Kinematic {
                position: screen_origin(),
                velocity: Vec2::ZERO,
                acceleration: Vec2::ZERO,
            },
            lives: 3,
            orientation: 0.0,
        };
    }
    pub fn position(&self) -> Vec2 {
        return self.kinematic.position;
    }
    pub fn velocity(&self) -> Vec2 {
        return self.kinematic.velocity;
    }
    pub fn acceleration(&self) -> Vec2 {
        return self.kinematic.acceleration;
    }
    pub fn rotation_matrix(&self) -> Mat2 {
        return mat2(
            //     top row                  bottom row
            vec2(self.orientation.cos(), self.orientation.sin()), // left column
            vec2(-self.orientation.sin(), self.orientation.cos()), // right column
        );
    }
    pub fn vertices(&self) -> (Vec2, Vec2, Vec2) {
        let rotation = self.rotation_matrix();
        let center = self.kinematic.position + (rotation * Self::POSITION_OFFSET);

        let front = center + (rotation * Self::FRONT_OFFSET);
        let left = center + (rotation * Self::LEFT_OFFSET);
        let right = center + (rotation * Self::RIGHT_OFFSET);

        return (front, left, right);
    }
}
impl Player {
    pub fn keep_on_screen(&mut self) {
        let next_position = self.kinematic.position + self.kinematic.velocity;
        if next_position.x < 0.0 {
            self.kinematic.position.x = screen_width();
        }
        if next_position.x > screen_width() {
            self.kinematic.position.x = 0.0;
        }
        if next_position.y < 0.0 {
            self.kinematic.position.y = screen_height();
        }
        if next_position.y > screen_height() {
            self.kinematic.position.y = 0.0;
        }
    }
    pub fn handle_input(&mut self) {
        if is_key_down(KeyCode::Left) {
            self.orientation -= Self::ROTATION_DELTA;
        }
        if is_key_down(KeyCode::Right) {
            self.orientation += Self::ROTATION_DELTA;
        }
        if is_key_down(KeyCode::Up) {
            let forward = self.rotation_matrix() * vec2(0.0, 0.2);

            // apply acceleration (using linear interpolation aka lerp)
            self.kinematic.acceleration = self.kinematic.acceleration.lerp(forward, 0.1);
        }

        if self.kinematic.velocity.length_squared() > Self::MAX_SPEED_SQUARED {
            self.kinematic.velocity = Self::MAX_SPEED * self.kinematic.velocity.normalize();
        }

        self.kinematic.apply_friction();
        self.keep_on_screen();
    }
    pub fn draw(&self) {
        let (v1, v2, v3) = self.vertices();
        draw_triangle(v1, v2, v3, WHITE);
        draw_circle(
            self.kinematic.position.x,
            self.kinematic.position.y,
            2.5,
            RED,
        );
    }
}
