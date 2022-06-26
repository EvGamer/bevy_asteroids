use bevy::math::const_vec3;
use bevy::prelude::{Entity, Query, Res, Time, Transform};
use crate::components::{ForwardAcceleration, Velocity};

pub fn forward_acceleration_system(
  mut query: Query<(Entity, &mut Velocity, &ForwardAcceleration, &Transform)>,
  time: Res<Time>,
) {
  for (_, mut velocity, acceleration, transform) in query.iter_mut() {
    let dt_sec = time.delta_seconds();
    let angle = transform.rotation;
    let dv = angle.mul_vec3(const_vec3!([acceleration.acceleration, 0., 0.]));

    velocity.x += dv.x * dt_sec;
    velocity.y += dv.y * dt_sec;
  }
}