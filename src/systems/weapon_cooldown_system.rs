use bevy::prelude::{Query, Res, Time};
use crate::components::Weapon;

pub fn weapon_cooldown_system(mut query: Query<&mut Weapon>, time: Res<Time>) {
  for mut weapon in query.iter_mut() {
    weapon.cooldown.tick(time.delta());
  }
}
