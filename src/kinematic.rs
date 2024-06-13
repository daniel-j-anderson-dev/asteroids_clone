use macroquad::prelude::*;

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
    pub fn lerp_acceleration(&mut self, acceleration: Vec2) {
        self.acceleration = self.acceleration.lerp(acceleration, 0.1);
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
    pub fn step_motion(&mut self) {
        let p_0 = self.position.clone();
        let v_0 = self.velocity.clone();
        let a_0 = self.acceleration.clone();

        let p_1 = p_0 + v_0;
        let v_1 = v_0 + a_0;

        self.position = p_1;
        self.velocity = v_1;
    }
    pub fn step_friction(&mut self) {
        // apply friction (using linear interpolation with <0, 0> aka lerp)
        self.acceleration += (Vec2::ZERO - self.acceleration) * 0.02;
        self.velocity += (Vec2::ZERO - self.velocity) * 0.02;
    }
}
