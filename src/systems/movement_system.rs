use bevy::prelude::{Query, Res, Time, Transform};
use crate::components::Velocity;

pub fn movement_system(
  mut query: Query<(&Velocity, &mut Transform)>,
  time: Res<Time>,
) {
  for (velocity, mut transform) in query.iter_mut() {
    let translation = &mut transform.translation;
    let dt_sec = time.delta_seconds();
    translation.x += velocity.x * dt_sec;
    translation.y += velocity.y * dt_sec;
  }
}