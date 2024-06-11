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

    const VERTEX1: Vec2 = vec2(-Self::HALF_TRIANGLE_SIZE, Self::TRIANGLE_SIZE);
    const VERTEX2: Vec2 = vec2(Self::HALF_TRIANGLE_SIZE, Self::TRIANGLE_SIZE);        
    const VERTEX3: Vec2 = vec2(Self::TRIANGLE_SIZE, 0.0);    
    const VERTEX4: Vec2 = vec2(Self::HALF_TRIANGLE_SIZE, -Self::TRIANGLE_SIZE);    
    const VERTEX5: Vec2 = vec2(-Self::HALF_TRIANGLE_SIZE, -Self::TRIANGLE_SIZE);    
    const VERTEX6: Vec2 = vec2(-Self::TRIANGLE_SIZE / 2.0, 0.0);    
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
    pub fn vertices(&self) -> (Vec2, Vec2, Vec2, Vec2, Vec2, Vec2) {
        let rotation = self.rotation_matrix();

        let vertex1 = (rotation * Self::VERTEX1) + self.kinematic.position;    
        let vertex2 = (rotation * Self::VERTEX2) + self.kinematic.position;
        let vertex3 = (rotation * Self::VERTEX3) + self.kinematic.position;
        let vertex4 = (rotation * Self::VERTEX4) + self.kinematic.position;
        let vertex5 = (rotation * Self::VERTEX5) + self.kinematic.position;
        let vertex6 = (rotation * Self::VERTEX6) + self.kinematic.position;
        
        return (vertex1, vertex2, vertex3, vertex4, vertex5, vertex6);
    }
}