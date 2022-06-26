use bevy::prelude::{ Component, Deref, DerefMut };
use bevy::math::Vec2;

#[derive(Component, Deref, DerefMut)]
pub struct Velocity(pub Vec2);