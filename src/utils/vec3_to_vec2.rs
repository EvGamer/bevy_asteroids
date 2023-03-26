use bevy::math::{Vec3, Vec2};

pub fn vec3_to_vec2(vector: &Vec3) -> Vec2 {
  return Vec2::from_array([vector.x, vector.y])
}