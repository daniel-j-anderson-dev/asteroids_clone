use macroquad::prelude::*;

pub struct Player {
    pub kinematic: Kinematic,
    pub lives: usize,
    pub orientation: f32,
}
impl Player {
    pub const ROTATION_DELTA: f32 = 0.1;
    pub const SIZE: f32 = 20.0;
    const MAX_SPEED: f32 = 3.0;
    const POSITION_OFFSET: Vec2 = vec2(0.0, Self::SIZE / - 4.0);
    const FRONT_OFFSET: Vec2 = vec2(0.0, Self::SIZE);
    const LEFT_OFFSET: Vec2 = vec2(-Self::SIZE / 2.0, 0.0);
    const RIGHT_OFFSET: Vec2 = vec2(Self::SIZE / 2.0, 0.0);
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
}
impl Player {
    pub fn handle_input(&mut self) {
        if is_key_down(KeyCode::Left) {
            self.orientation -= Self::ROTATION_DELTA;
        }
        if is_key_down(KeyCode::Right) {
            self.orientation += Self::ROTATION_DELTA;
        }
        if is_key_down(KeyCode::Up) {
            self.kinematic.acceleration = self.rotation_matrix() * vec2(0.0, 1.0);
        } else {
            self.kinematic.acceleration = Vec2::ZERO;
        }
        // if the current speed exceeds the max then move velocity towards zero a little
        if self.kinematic.velocity.length() > Self::MAX_SPEED {
            self.kinematic.velocity = self.kinematic.velocity.lerp(Vec2::ZERO, 0.1);
        }
    }
    pub fn rotation_matrix(&self) -> Mat2 {
        return mat2( //     top row                  bottom row
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
    pub fn draw(&self) {
        let (v1, v2, v3) = self.vertices();
        draw_triangle(v1, v2, v3, WHITE);
        draw_circle(self.kinematic.position.x, self.kinematic.position.y, 2.5, RED);
    }
}

pub struct Kinematic {
    pub position: Vec2,
    pub velocity: Vec2,
    pub acceleration: Vec2,
}
impl Kinematic {
    pub fn step(&mut self) {
        self.velocity += self.acceleration;
        self.position += self.velocity;
    }
}

pub fn screen_dimensions() -> Vec2 {
    return vec2(screen_width(), screen_height());
}
pub fn screen_origin() -> Vec2 {
    return screen_dimensions() / 2.0;
}
