use bevy::prelude::*;

mod systems;

use systems::setup_system::*;

const BACKGROUND_COLOR: Color = Color::rgb(0.0, 0.0, 0.0);

fn main() {
    App::new()
      .add_plugins(DefaultPlugins)
      .insert_resource(ClearColor(BACKGROUND_COLOR))
      .add_startup_system(setup_system)
      .add_system(bevy::input::system::exit_on_esc_system)
      .run();
}
