use std::f32::consts::PI;
use bevy::input::Input;
use bevy::prelude::{KeyCode, Quat, Query, Res, Time, Transform, With};
use crate::components::{ForwardAcceleration};
use crate::resources::KeyboardSettings;
use super::super::components::Player;

const ROTATION_ANGLE: f32 = PI;
const ACCELERATION: f32 = 200.;

pub fn player_control_system(
  mut query: Query<(&mut ForwardAcceleration, &mut Transform), With<Player>>,
  keyboard: Res<Input<KeyCode>>,
  settings: Res<KeyboardSettings>,
  time: Res<Time>
) {
  if let Ok((mut acceleration, mut transform)) = query.get_single_mut() {
    let is_pressed = |&key| keyboard.pressed(key);

    if settings.forward.iter().any(is_pressed) {
      acceleration.acceleration = ACCELERATION;
    } else if settings.backward.iter().any(is_pressed) {
      acceleration.acceleration = -ACCELERATION;
    } else {
      acceleration.acceleration = 0.;
    }

    let dt = time.delta_seconds();

    if settings.rotate_left.iter().any(is_pressed) {
      transform.rotation *= Quat::from_rotation_z(ROTATION_ANGLE * dt);
    }
    if settings.rotate_right.iter().any(is_pressed) {
      transform.rotation *= Quat::from_rotation_z(-ROTATION_ANGLE * dt);
    }
  }
}