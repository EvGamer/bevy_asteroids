use bevy::prelude::{ Handle, Image };

pub struct Textures {
  pub player_ship: Handle<Image>,
  pub player_projectile: Handle<Image>,
  pub asteroid: Handle<Image>
}