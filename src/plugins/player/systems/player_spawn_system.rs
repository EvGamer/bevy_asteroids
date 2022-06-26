use bevy::math::const_vec2;
use bevy::prelude::{AssetServer, Commands, default, Quat, Res, SpriteBundle, Transform, Vec3};
use crate::components::{ForwardAcceleration, Velocity};
use super::super::components::player::Player;

pub fn player_spawn_system(mut commands: Commands, asset_server: Res<AssetServer>) {
  commands.spawn()
    .insert(Player)
    .insert(Velocity(const_vec2!([0., 0.])))
    .insert(ForwardAcceleration { acceleration: 10. })
    .insert_bundle(SpriteBundle {
      texture: asset_server.load("images/player_ship.png"),
      transform: Transform {
        translation: Vec3::new(0., 0., 0.),
        rotation: Quat::from_rotation_z(f32::to_radians(90.)),
        ..default()
      },
      ..default()
    });
}