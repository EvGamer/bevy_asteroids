use bevy::ui::entity::{ UiCameraBundle };
use bevy::ecs::system::{ Commands };
use bevy::prelude::{ OrthographicCameraBundle };

pub fn setup_system(mut commands: Commands) {
  commands.spawn_bundle(OrthographicCameraBundle::new_2d());
  commands.spawn_bundle(UiCameraBundle::default());
}
