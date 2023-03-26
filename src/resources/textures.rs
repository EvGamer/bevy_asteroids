use bevy::prelude::{ Handle, Image, Resource };

#[derive(Resource)]
pub struct Textures {
  pub player_ship: Handle<Image>,
  pub player_projectile: Handle<Image>,
  pub asteroid: Handle<Image>
}