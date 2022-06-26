use bevy::prelude::*;

mod systems;
mod plugins;
mod components;
mod resources;

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
      })
      .add_startup_system(setup_system)
      .add_system(movement_system)
      .add_system(forward_acceleration_system)
      .add_system(bevy::input::system::exit_on_esc_system)
      .run();
}
