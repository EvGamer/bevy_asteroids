use bevy::math::{Vec3};
use bevy::prelude::{Quat};

pub fn vector_from_radial_coord(value: f32, rotation: &Quat) -> Vec3 {
  rotation.mul_vec3(Vec3::from_array([value, 0., 0.]))
}