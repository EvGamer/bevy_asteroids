use bevy::prelude::*;
// use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod systems;
mod plugins;
mod components;
mod resources;
mod utils;

use systems::*;
use plugins::*;
use resources::*;

const BACKGROUND_COLOR: Color = Color::rgb(0.0, 0.0, 0.0);

fn main() {
    App::new()
      .add_plugins(DefaultPlugins)
      .add_plugin(PlayerPlugin)
      .insert_resource(ClearColor(BACKGROUND_COLOR))
      .insert_resource(KeyboardSettings {
        forward: vec![KeyCode::W, KeyCode::Up],
        backward: vec![KeyCode::S, KeyCode::Down],
        rotate_left: vec![KeyCode::A, KeyCode::Left],
        rotate_right: vec![KeyCode::D, KeyCode::Right],
        fire: vec![KeyCode::Space, KeyCode::Numpad0],
      })
      .add_startup_system(setup_system)
      .add_system(movement_system)
      .add_system(forward_acceleration_system)
      .add_system(weapon_cooldown_system)
      .add_system(wrap_around_system)
      .add_system(bevy::window::close_on_esc)
      // .add_plugin(WorldInspectorPlugin)
      .run();
}
