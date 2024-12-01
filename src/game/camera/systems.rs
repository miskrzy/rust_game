use bevy::{
    math::Vec3,
    prelude::{Camera2dBundle, Commands, Query, Transform, Window, With, Without},
    window::PrimaryWindow,
};

use super::super::arena::constants::{HEIGHT as ARENA_HEIGHT, WIDTH as ARENA_WIDTH};
use super::super::player::components::Player;
use super::components::GameCamera;
use super::constants::{ARENA_HEIGHT_OFFSET, ARENA_WIDTH_OFFSET, DEPTH};

pub fn spawn(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();
    let x_position = window.width() / 2.;
    let y_position = window.height() / 2.;
    let z_position: f32 = DEPTH;

    let camera_bundle = Camera2dBundle {
        transform: Transform::from_xyz(x_position, y_position, z_position),
        ..Default::default()
    };

    commands.spawn((camera_bundle, GameCamera));
}

pub fn follow_player(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<GameCamera>, Without<Player>)>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        if let Ok(mut transform) = camera_query.get_single_mut() {
            transform.translation = player_transform.translation;
        }
    }
}

pub fn restrict_movement(
    mut camera_query: Query<&mut Transform, With<GameCamera>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    let window_position = window.size() / 2.;
    if let Ok(mut transform) = camera_query.get_single_mut() {
        let min_vec = Vec3 {
            x: window_position.x - ARENA_WIDTH / 2. + (window_position.x - ARENA_WIDTH_OFFSET),
            y: window_position.y - ARENA_HEIGHT / 2. + (window_position.y - ARENA_HEIGHT_OFFSET),
            z: DEPTH,
        };
        let max_vec = Vec3 {
            x: window_position.x + ARENA_WIDTH / 2. - (window_position.x - ARENA_WIDTH_OFFSET),
            y: window_position.y + ARENA_HEIGHT / 2. - (window_position.y - ARENA_HEIGHT_OFFSET),
            z: DEPTH,
        };
        transform.translation = transform.translation.clamp(min_vec, max_vec);
    }
}
