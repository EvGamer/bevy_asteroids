use bevy::math::const_vec3;
use bevy::prelude::{Query, Res, Time, Transform};
use crate::components::{ForwardAcceleration, Velocity};
use crate::utils::vector_from_radial_coord;

pub fn forward_acceleration_system(
  mut query: Query<(&mut Velocity, &ForwardAcceleration, &Transform)>,
  time: Res<Time>,
) {
  for (mut velocity, acceleration, transform) in query.iter_mut() {
    let dt_sec = time.delta_seconds();
    let dv = vector_from_radial_coord(acceleration.acceleration, &transform.rotation);

    velocity.x += dv.x * dt_sec;
    velocity.y += dv.y * dt_sec;
  }
}