use bevy::math::{const_vec2, Vec3, Vec2};

pub fn vec3_to_vec2(vector: &Vec3) -> Vec2 {
  const_vec2!([vector.x, vector.y])
}