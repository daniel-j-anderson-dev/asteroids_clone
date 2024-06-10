use macroquad::prelude::*;

pub struct Kinematic {
    pub position: Vec2,
    pub velocity: Vec2,
    pub acceleration: Vec2,
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
        // apply friction (using linear interpolation aka lerp)
        self.acceleration = self.acceleration.lerp(Vec2::ZERO, 0.02);
        self.velocity = self.velocity.lerp(Vec2::ZERO, 0.02);
    }
}
