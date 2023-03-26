use bevy::prelude::{Query, Res, Time};
use crate::components::WeaponCooldown;

pub fn weapon_cooldown_system(mut query: Query<&mut WeaponCooldown>, time: Res<Time>) {
  for mut weapon_cooldown in query.iter_mut() {
    weapon_cooldown.timer.tick(time.delta());
  }
}
