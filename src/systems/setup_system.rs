use bevy::ecs::system::{ Commands };
use bevy::prelude::{AssetServer, Camera2dBundle, Res, ResMut, Windows};
use crate::resources::Textures;

pub fn setup_system(
  mut commands: Commands,
  asset_server: Res<AssetServer>,
  mut windows: ResMut<Windows>
) {
  let window = windows.get_primary_mut().unwrap();
  window.set_resizable(false);
  window.set_resolution(1280., 720.);

  commands.spawn(Camera2dBundle::default());

  commands.insert_resource(Textures {
    player_ship: asset_server.load("images/player_ship.png"),
    player_projectile: asset_server.load("images/red_laser.png"),
    asteroid: asset_server.load("images/asteroid.png"),
  })
}
