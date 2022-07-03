use bevy::math::{const_vec3, Vec3};
use bevy::prelude::{Quat};

use super::vec3_to_vec2;

pub fn vector_from_radial_coord(value: f32, rotation: &Quat) -> Vec3 {
  rotation.mul_vec3(const_vec3!([value, 0., 0.]))
}