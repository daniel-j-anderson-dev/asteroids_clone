use macroquad::prelude::*;

/// This trait provides default implementations of [Kinematic]'s getters if `Self` can provide a reference to a [Kinematic]
pub trait KinematicGetters {
    fn kinematic(&self) -> &Kinematic;
    fn position(&self) -> Vec2 {
        return self.kinematic().position;
    }
    fn velocity(&self) -> Vec2 {
        return self.kinematic().velocity;
    }
    fn acceleration(&self) -> Vec2 {
        return self.kinematic().acceleration;
    }
}

/// This struct is responsible for movement in 
pub struct Kinematic {
    position: Vec2,
    velocity: Vec2,
    acceleration: Vec2,
}
impl Kinematic {
    pub fn new(position: Vec2, velocity: Vec2, acceleration: Vec2) -> Self {
        return Self {
            position,
            velocity,
            acceleration,
        };
    }
}
impl KinematicGetters for Kinematic {
    fn kinematic(&self) -> &Kinematic {
        return self;
    }
}
impl Kinematic {
    pub fn apply_acceleration(&mut self, acceleration: Vec2) {
        self.acceleration = self.acceleration.move_towards(acceleration, 0.01);
    }

    pub fn cap_speed(&mut self, max_speed: f32) {
        if self.velocity.length() > max_speed {
            self.velocity = self.velocity.normalize() * max_speed;
        }
    }

    pub fn keep_on_screen(&mut self) {
        // take a peek forward in time!
        let next_position = self.position + self.velocity;

        // next frame player will travel off the left side
        if next_position.x < 0.0 {
            // so lets teleport them to the right side
            self.position.x = screen_width();
        }
        if next_position.x > screen_width() {
            self.position.x = 0.0;
        }
        if next_position.y < 0.0 {
            self.position.y = screen_height();
        }
        if next_position.y > screen_height() {
            self.position.y = 0.0;
        }
    }

    /// Calculates and applies the next `position` and `velocity` using [Euler's Method](https://en.wikipedia.org/wiki/Euler_method)
    pub fn step_motion(&mut self) {
        let next_position = self.position + self.velocity;
        let next_velocity = self.velocity + self.acceleration;

        self.position = next_position;
        self.velocity = next_velocity;
    }
    
    pub fn step_friction(&mut self) {
        // apply friction (using linear interpolation with <0, 0> aka lerp)
        self.acceleration += (Vec2::ZERO - self.acceleration) * 0.02;
        self.velocity += (Vec2::ZERO - self.velocity) * 0.02;
    }
}
