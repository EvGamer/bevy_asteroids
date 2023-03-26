use bevy::prelude::{ Res, Windows, Transform, Query };

pub fn wrap_around_system(
  mut query: Query<&mut Transform>,
  windows: Res<Windows>,
) {
  let window = windows.get_primary().unwrap();
  let width = window.width();
  let height = window.height();
  let half_width = width * 0.5;
  let half_height = height * 0.5;

  for mut transform in query.iter_mut() {
    let translation = &mut transform.translation;

    if translation.x > half_width{
      translation.x -= width;
    }
    else if translation.x < - half_width {
      translation.x += width;
    }
    else if translation.y > half_height {
      translation.y -= height;
    }
    else if translation.y < -half_height {
      translation.y += height;
    }
  }
}