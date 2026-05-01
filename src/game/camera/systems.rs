use bevy::{
    math::{Vec2, Vec3},
    prelude::{Camera2d, Commands, Query, Transform, Window, With, Without},
    window::PrimaryWindow,
};

use super::super::arena::constants::{HEIGHT as ARENA_HEIGHT, WIDTH as ARENA_WIDTH};
use super::super::player::components::Player;
use super::components::GameCamera;
use super::constants::{ARENA_HEIGHT_OFFSET, ARENA_WIDTH_OFFSET, DEPTH};

pub fn spawn(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.single().unwrap();
    let x_position = window.width() / 2.;
    let y_position = window.height() / 2.;
    let z_position: f32 = DEPTH;

    commands.spawn((
        Camera2d,
        Transform::from_xyz(x_position, y_position, z_position),
        GameCamera,
    ));
}

pub fn follow_player(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<GameCamera>, Without<Player>)>,
) {
    if let Ok(player_transform) = player_query.single() {
        if let Ok(mut transform) = camera_query.single_mut() {
            transform.translation = player_transform.translation;
        }
    }
}

pub fn restrict_movement(
    mut camera_query: Query<&mut Transform, With<GameCamera>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.single().unwrap();
    let window_center = window.size() / 2.;
    let window_size = window.size();
    let arena_size = Vec2::new(ARENA_WIDTH, ARENA_HEIGHT);
    let bottom_left_arena_corner = window_center - (arena_size / 2.);
    let top_right_arena_corner = window_center + (arena_size / 2.);
    let camera_min_shift_from_edge =
        window_size / 2. - Vec2::new(ARENA_WIDTH_OFFSET, ARENA_HEIGHT_OFFSET);
    let bottom_left_camera_limit =
        (bottom_left_arena_corner + camera_min_shift_from_edge).min(window_center);
    let top_right_camera_limit =
        (top_right_arena_corner - camera_min_shift_from_edge).max(window_center);
    if let Ok(mut transform) = camera_query.single_mut() {
        let min_vec = Vec3 {
            x: bottom_left_camera_limit.x,
            y: bottom_left_camera_limit.y,
            z: DEPTH,
        };
        let max_vec = Vec3 {
            x: top_right_camera_limit.x,
            y: top_right_camera_limit.y,
            z: DEPTH,
        };
        transform.translation = transform.translation.clamp(min_vec, max_vec);
    }
}
