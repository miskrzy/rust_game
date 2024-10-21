use bevy::{
    prelude::{Camera2dBundle, Commands, Query, Transform, Window, With},
    window::PrimaryWindow,
};

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();
    let x_position = window.width() / 2.0;
    let y_position = window.height() / 2.0;
    let z_position: f32 = 0.0;

    let camera_bundle = Camera2dBundle {
        transform: Transform::from_xyz(x_position, y_position, z_position),
        ..Default::default()
    };

    commands.spawn(camera_bundle);
}
