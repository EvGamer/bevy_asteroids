use bevy::ui::entity::{ UiCameraBundle, TextBundle };
use bevy::ecs::system::{ Commands, Res };
use bevy::asset::{ AssetServer };
use bevy::math::{ Rect };
use bevy::prelude::{Color, default, OrthographicCameraBundle, PositionType, Style, Val};
use bevy::text::{Text, TextSection, TextStyle};

pub fn setup_system(mut commands: Commands, asset_server: Res<AssetServer>) {
  commands.spawn_bundle(OrthographicCameraBundle::new_2d());
  commands.spawn_bundle(UiCameraBundle::default());

  commands.spawn_bundle(TextBundle {
    text: Text {
      sections: vec![
        TextSection {
          value: "Hello World".to_string(),
          style: TextStyle {
            font: asset_server.load("fonts/FiraMono-Medium.ttf"),
            font_size: 40.0,
            color: Color::rgb(1.0, 1.0, 1.0),
          },
        },
      ],
      ..default()
    },
    style: Style {
      position_type: PositionType::Absolute,
      position: Rect {
        top: Val::Px(20.0),
        left: Val::Px(20.0),
        ..default()
      },
      ..default()
    },
    ..default()
  });
}
