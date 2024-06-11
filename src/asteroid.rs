use macroquad::prelude::*;
use crate::Kinematic;

//! Implement an asteroid struct that is drawn as a spinning hexagon with constant velocity
//! random position constructor!
//! - figure out where vertexes are relative to the origin
//! - rotate those vertexes according to the rotation matrix
//! - add those rotated vertexes to the position of the hexagon
//! - draw a line between each vertex

pub struct Asteroid {
    kinematic: Kinematic,
    size: f32,
    orientation: f32,
}
impl Asteroid {
    const SIZE: f32 = 20.0;

    const TRIANGLE_SIZE: f32 = Self::SIZE / 2.0;
    const HALF_TRIANGLE_SIZE: f32 = Self::TRIANGLE_SIZE / 2.0;

    const VERTEX_ONE: Vec2 = vec2(-Self::HALF_TRIANGLE_SIZE, Self::TRIANGLE_SIZE);
    const VERTEX_TWO: Vec2 = vec2(Self::HALF_TRIANGLE_SIZE, Self::TRIANGLE_SIZE);        
    const VERTEX_THREE: Vec2 = vec2(Self::TRIANGLE_SIZE, 0.0);    
    const VERTEX_FOUR: Vec2 = vec2(Self::HALF_TRIANGLE_SIZE, -Self::TRIANGLE_SIZE);    
    const VERTEX_FIVE: Vec2 = vec2(-Self::HALF_TRIANGLE_SIZE, -Self::TRIANGLE_SIZE);    
    const VERTEX_SIX: Vec2 = vec2(-Self::TRIANGLE_SIZE / 2.0, 0.0);    
}
impl Asteroid {
    pub fn position(&self) -> Vec2 {
        return self.kinematic.position;
    }
    pub fn velocity(&self) -> Vec2 {
        return self.kinematic.velocity;
    }
    pub fn acceleration(&self) -> Vec2 {
        return self.kinematic.acceleration;
    }
    pub fn orientation(&self) -> f32 {
        return self.orientation;
    }
    pub fn rotation_matrix(&self) -> Mat2 {
        return mat2(
            //     top row                  bottom row
            vec2(self.orientation.cos(), self.orientation.sin()), // left column
            vec2(-self.orientation.sin(), self.orientation.cos()), // right column
        );
    }
    pub fn verticies(&self) -> (Vec2, Vec2, Vec2, Vec2, Vec2, Vec2) {
        let rotation = self.rotation_matrix();

        let vertex_one = (rotation * Self::VERTEX_ONE) + self.kinematic.position;    
        let vertex_two = (rotation * Self::VERTEX_TWO) + self.kinematic.position;
        let vertex_three = (rotation * Self::VERTEX_THREE) + self.kinematic.position;
        let vertex_four = (rotation * Self::VERTEX_FOUR) + self.kinematic.position;
        let vertex_five = (rotation * Self::VERTEX_FIVE) + self.kinematic.position;
        let vertex_six = (rotation * Self::VERTEX_SIX) + self.kinematic.position;
        
        return (vertex_one, vertex_two, vertex_three, vertex_four, vertex_five, vertex_six);
    }
}