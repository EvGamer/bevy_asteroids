use bevy::prelude::{AssetServer, Commands, default, Res, SpriteBundle, Transform, Vec3};
use super::super::components::player::Player;

pub fn player_spawn_system(mut commands: Commands, asset_server: Res<AssetServer>) {
  commands.spawn()
    .insert(Player)
    .insert_bundle(SpriteBundle {
      texture: asset_server.load("images/player_ship.png"),
      transform: Transform {
        translation: Vec3::new(0., 30., 0.),
        ..default()
      },
      ..default()
    });
}