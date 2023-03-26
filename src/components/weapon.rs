use std::time::{ Duration };
use bevy::prelude::{ Component };
use bevy::time::{ Timer, TimerMode };

#[derive(Component, Default)]
pub struct Weapon {
  pub cooldown: Timer,
}

impl Weapon {
  pub fn new(cooldown_sec: f32) -> Self {
    let mut weapon = Weapon {
      cooldown: Timer::from_seconds(cooldown_sec, TimerMode::Once)
    };
    // if weapon wasn't fired before, there should be no cooldown
    weapon.cooldown.tick(Duration::from_secs_f32(cooldown_sec));
    return weapon;
  }
}