use bevy::{
    asset::AssetServer,
    input::ButtonInput,
    math::Vec3,
    prelude::{Commands, KeyCode, Query, Res, Transform, Window, With},
    sprite::SpriteBundle,
    time::Time,
    window::PrimaryWindow,
};

use super::components::Player;
use super::constants::{PLAYER_PICTURE_PATH, PLAYER_SPEED};

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();
    let x_position = window.width() / 2.0;
    let y_position = window.height() / 2.0;
    let z_position: f32 = 0.0;

    let player_texture = asset_server.load(PLAYER_PICTURE_PATH);

    let player_sprite_bundle = SpriteBundle {
        transform: Transform::from_xyz(x_position, y_position, z_position),
        texture: player_texture,
        ..Default::default()
    };

    commands.spawn((player_sprite_bundle, Player));
}

pub fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    let mut player_transform = player_query.get_single_mut().unwrap();

    let mut direction = Vec3::ZERO;

    if keyboard_input.pressed(KeyCode::ArrowLeft) || keyboard_input.pressed(KeyCode::KeyA) {
        direction += Vec3::new(-1.0, 0.0, 0.0);
    }
    if keyboard_input.pressed(KeyCode::ArrowRight) || keyboard_input.pressed(KeyCode::KeyD) {
        direction += Vec3::new(1.0, 0.0, 0.0);
    }
    if keyboard_input.pressed(KeyCode::ArrowUp) || keyboard_input.pressed(KeyCode::KeyW) {
        direction += Vec3::new(0.0, 1.0, 0.0);
    }
    if keyboard_input.pressed(KeyCode::ArrowDown) || keyboard_input.pressed(KeyCode::KeyS) {
        direction += Vec3::new(0.0, -1.0, 0.0);
    }

    direction = direction.normalize_or_zero();

    player_transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
}
