use bevy::prelude::{Commands, Vec2, Vec3, Quat, Res, SpriteBundle, Transform, default, Sprite};
use crate::components::Velocity;
use crate::player::components::PlayerProjectile;
use crate::resources::Textures;
use crate::utils::{vec3_to_vec2, vector_from_radial_coord};

const PROJECTILE_VELOCITY_ABS: f32 = 400.0;

pub fn spawn_laser (
  mut commands: Commands,
  textures: Res<Textures>,
  position: Vec3,
  rotation: Quat
) {
  commands.spawn(PlayerProjectile)
    .insert(Velocity(vec3_to_vec2(&vector_from_radial_coord(
      PROJECTILE_VELOCITY_ABS,
      &rotation,
    ))))
    .insert(SpriteBundle {
      texture: textures.player_projectile.clone(),
      sprite: Sprite {
        custom_size: Some(Vec2::new(42.0, 7.0)),
        ..default()
      },
      transform: Transform {
        translation: Vec3::new(position.x, position.y, 0.),
        rotation: rotation.clone(),
        ..default()
      },
      ..default()
    })
  ;
}