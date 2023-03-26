use std::time::{ Duration };
use bevy::prelude::{ Component };
use bevy::time::{ Timer, TimerMode };

#[derive(Component, Default)]
pub struct WeaponCooldown {
  pub timer: Timer,
}

impl WeaponCooldown {
  pub fn new(cooldown_sec: f32) -> Self {
    let mut weapon_cooldown = WeaponCooldown {
      timer: Timer::from_seconds(cooldown_sec, TimerMode::Once)
    };
    // if weapon wasn't fired before, there should be no cooldown
    weapon_cooldown.timer.tick(Duration::from_secs_f32(cooldown_sec));
    return weapon_cooldown;
  }
}