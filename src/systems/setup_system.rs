use bevy::ui::entity::{ UiCameraBundle };
use bevy::ecs::system::{ Commands };
use bevy::prelude::{AssetServer, OrthographicCameraBundle, Res};
use crate::resources::Textures;

pub fn setup_system(
  mut commands: Commands,
  asset_server: Res<AssetServer>,
) {
  commands.spawn_bundle(OrthographicCameraBundle::new_2d());
  commands.spawn_bundle(UiCameraBundle::default());

  commands.insert_resource(Textures {
    player_ship: asset_server.load("images/player_ship.png"),
    player_projectile: asset_server.load("images/red_laser"),
    asteroid: asset_server.load("images/asteroid"),
  })
}
