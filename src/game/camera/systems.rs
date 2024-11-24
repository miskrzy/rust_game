use bevy::{
    prelude::{Camera2dBundle, Commands, Query, Transform, Window, With, Without},
    window::PrimaryWindow,
};

use crate::game::player::components::Player;

use super::components::GameCamera;

pub fn spawn(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();
    let x_position = window.width() / 2.0;
    let y_position = window.height() / 2.0;
    let z_position: f32 = 0.0;

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
