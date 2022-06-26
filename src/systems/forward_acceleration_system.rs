use bevy::math::const_vec3;
use bevy::prelude::{Query, Res, Time, Transform};
use crate::components::{ForwardAcceleration, Velocity};

pub fn forward_acceleration_system(
  mut query: Query<(&mut Velocity, &ForwardAcceleration, &Transform)>,
  time: Res<Time>,
) {
  for (mut velocity, acceleration, transform) in query.iter_mut() {
    let dt_sec = time.delta_seconds();
    let dv = transform.rotation.mul_vec3(const_vec3!([acceleration.acceleration, 0., 0.]));

    velocity.x += dv.x * dt_sec;
    velocity.y += dv.y * dt_sec;
  }
}