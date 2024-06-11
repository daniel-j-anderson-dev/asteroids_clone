use macroquad::prelude::*;

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
    pub fn position(&self) -> Vec2 {
        return self.position;
    }
    pub fn velocity(&self) -> Vec2 {
        return self.velocity;
    }
    pub fn acceleration(&self) -> Vec2 {
        return self.acceleration;
    }
}
impl Kinematic {
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
