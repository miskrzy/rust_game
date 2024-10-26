use bevy::{
    asset::AssetServer,
    math::Vec2,
    prelude::{Commands, Query, Res, Transform, With, Without},
    sprite::{Sprite, SpriteBundle},
    time::Time,
    window::{PrimaryWindow, Window},
};
use rand::{thread_rng, Rng};

use super::constants::{ENEMY_PICTURE_PATH, ENEMY_SPEED, ENEMY_SPRITE_DIAMETER};

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

        let sprite = Sprite {
            custom_size: Some(Vec2::new(ENEMY_SPRITE_DIAMETER, ENEMY_SPRITE_DIAMETER)),
            ..Default::default()
        };

        let enemy_sprite_bundle = SpriteBundle {
            sprite: sprite,
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

    for mut enemy_transform in enemy_query.iter_mut() {
        let distance = player_translation - enemy_transform.translation;
        let direction = distance.normalize_or_zero();
        let mut translation_change = direction * ENEMY_SPEED * time.delta_seconds();
        translation_change = if translation_change.length() > distance.length() {
            distance
        } else {
            translation_change
        };
        enemy_transform.translation += translation_change;
    }
}

pub fn restrict_enemy_movement(
    mut enemy_query: Query<&mut Transform, With<Enemy>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    for mut enemy_transform in enemy_query.iter_mut() {
        let enemy_radius = ENEMY_SPRITE_DIAMETER / 2.0;
        let adjusted_x = enemy_transform
            .translation
            .x
            .clamp(enemy_radius, window.width() - enemy_radius);
        let adjusted_y = enemy_transform
            .translation
            .y
            .clamp(enemy_radius, window.height() - enemy_radius);
        enemy_transform.translation.x = adjusted_x;
        enemy_transform.translation.y = adjusted_y;
    }
}
