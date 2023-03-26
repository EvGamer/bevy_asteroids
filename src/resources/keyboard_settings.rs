use bevy::prelude::{ KeyCode, Resource };

#[derive(Resource)]
pub struct KeyboardSettings {
  pub forward: Vec<KeyCode>,
  pub backward: Vec<KeyCode>,
  pub rotate_left: Vec<KeyCode>,
  pub rotate_right: Vec<KeyCode>,
  pub fire: Vec<KeyCode>,
}