use bevy::prelude::{ Plugin, App, StartupStage };

mod systems;
pub mod factories;
pub mod components;

use systems::player_spawn_system::*;
use systems::player_control_system::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_startup_system_to_stage(StartupStage::PostStartup, player_spawn_system)
      .add_system(player_control_system);
  }
}