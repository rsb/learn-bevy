use bevy::prelude::*;

fn main() {
  App::new()
    .add_plugins(DefaultPlugins)
    .add_systems(Startup, setup)
    .run();
}

#[derive(Component)]
pub struct Player {}

pub fn setup(mut cmds: Commands, asset_server: Res<AssetServer>) {
  cmds.spawn(Camera2dBundle::default());

  cmds.spawn((
    SpriteBundle {
      texture: asset_server.load("sprites/ball_blue_large.png"),
      ..default()
    },
    Player {},
  ));
}
