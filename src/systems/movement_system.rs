use bevy::prelude::{Entity, Query, Res, Time, Transform};
use crate::components::Velocity;

pub fn movement_system(
  mut query: Query<(Entity, &Velocity, &mut Transform)>,
  time: Res<Time>,
) {
  for (_, velocity, mut transform) in query.iter_mut() {
    let translation = &mut transform.translation;
    let dt_sec = time.delta_seconds();
    translation.x += velocity.x * dt_sec;
    translation.y += velocity.y * dt_sec;
  }
}