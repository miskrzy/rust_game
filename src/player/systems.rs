use bevy::{
    asset::AssetServer,
    input::ButtonInput,
    math::{Vec2, Vec3},
    prelude::{Commands, KeyCode, Query, Res, Transform, Window, With},
    sprite::{Sprite, SpriteBundle},
    time::Time,
    window::PrimaryWindow,
};

use super::components::{Health, Player};
use super::constants::{INITIAL_HEALTH, SPEED, SPRITE_DIAMETER, TEXTURE_PATH};

pub fn spawn(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server_resource: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();
    let x_position = window.width() / 2.;
    let y_position = window.height() / 2.;
    let z_position: f32 = 0.0;

    let texture = asset_server_resource.load(TEXTURE_PATH);
    let sprite = Sprite {
        custom_size: Some(Vec2::new(SPRITE_DIAMETER, SPRITE_DIAMETER)),
        ..Default::default()
    };

    let sprite_bundle = SpriteBundle {
        sprite: sprite,
        transform: Transform::from_xyz(x_position, y_position, z_position),
        texture: texture,
        ..Default::default()
    };

    let health = Health::initialize(INITIAL_HEALTH);

    commands.spawn((sprite_bundle, Player, health));
}

pub fn movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    let mut transform = player_query.get_single_mut().unwrap();

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

    transform.translation += direction * SPEED * time.delta_seconds();
}

pub fn restrict_movement(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    let mut transform = player_query.get_single_mut().unwrap();
    let radius = SPRITE_DIAMETER / 2.0;
    let adjusted_x = transform
        .translation
        .x
        .clamp(radius, window.width() - radius);
    let adjusted_y = transform
        .translation
        .y
        .clamp(radius, window.height() - radius);
    transform.translation.x = adjusted_x;
    transform.translation.y = adjusted_y;
}
