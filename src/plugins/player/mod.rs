use bevy::prelude::{ Plugin, App, StartupStage };

mod systems;
pub mod components;

use systems::player_spawn_system::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_startup_system_to_stage(StartupStage::PostStartup, player_spawn_system);
  }
}