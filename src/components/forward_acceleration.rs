use bevy::prelude::Component;

#[derive(Component)]
pub struct ForwardAcceleration {
  pub acceleration: f32,
}