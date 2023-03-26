use bevy::math::Vec2;
use bevy::prelude::{Commands, default, Quat, Res, SpriteBundle, Transform, Vec3};
use crate::components::{ForwardAcceleration, Velocity, Weapon};
use crate::resources::Textures;
use super::super::components::player::Player;

pub fn player_spawn_system(mut commands: Commands, textures: Res<Textures>) {
  commands.spawn(Player)
    .insert(Velocity(Vec2::from_array([0., 0.])))
    .insert(ForwardAcceleration { acceleration: 10. })
    .insert(Weapon::new(0.5))
    .insert(SpriteBundle {
      texture: textures.player_ship.clone(),
      transform: Transform {
        translation: Vec3::new(0., 0., 1.),
        rotation: Quat::from_rotation_z(f32::to_radians(90.)),
        ..default()
      },
      ..default()
    });
}