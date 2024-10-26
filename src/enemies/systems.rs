use bevy::{
    asset::AssetServer,
    prelude::{Commands, Query, Res, Transform, With, Without},
    sprite::SpriteBundle,
    time::Time,
    window::{PrimaryWindow, Window},
};
use rand::{thread_rng, Rng};

use super::constants::{ENEMY_PICTURE_PATH, ENEMY_SPEED};

use super::components::Enemy;

use crate::player::components::Player;

pub fn spawn_enemies(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let mut rng_gen = thread_rng();

    for _ in 0..4 {
        let window = window_query.get_single().unwrap();
        let x_position = rng_gen.gen_range(0.0..window.width());
        let y_position = rng_gen.gen_range(0.0..window.height());
        let z_position: f32 = 0.0;

        let enemy_texture = asset_server.load(ENEMY_PICTURE_PATH);

        let enemy_sprite_bundle = SpriteBundle {
            transform: Transform::from_xyz(x_position, y_position, z_position),
            texture: enemy_texture,
            ..Default::default()
        };

        commands.spawn((enemy_sprite_bundle, Enemy));
    }
}

pub fn enemies_movement(
    mut enemy_query: Query<&mut Transform, (With<Enemy>, Without<Player>)>,
    player_query: Query<&Transform, With<Player>>,
    time: Res<Time>,
) {
    let player_translation = player_query.get_single().unwrap().translation;

    for mut enemy in enemy_query.iter_mut() {
        let mut direction = player_translation - enemy.translation;
        if direction.length() > 0.0 {
            direction = direction.normalize();
        }
        enemy.translation += direction * ENEMY_SPEED * time.delta_seconds();
    }
}
