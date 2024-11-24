use crate::game::states::GameState;
use crate::states::AppState;

use super::super::projectiles::constants::CAST_SPEED;
use super::components::Score;
use super::constants::{INITIAL_HEALTH, SPEED, SPRITE_DEPTH, SPRITE_DIAMETER, TEXTURE_PATH};
use super::{
    components::{CastTimer, Health, Player},
    constants::HEALTH_REGEN,
};
use bevy::{
    asset::AssetServer,
    input::ButtonInput,
    math::{Vec2, Vec3},
    prelude::{Commands, Entity, KeyCode, NextState, Query, Res, ResMut, Transform, Window, With},
    sprite::{Sprite, SpriteBundle},
    time::{Time, Timer, TimerMode},
    window::PrimaryWindow,
};
use std::time::Duration;

pub fn spawn(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server_resource: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();
    let x_position = window.width() / 2.;
    let y_position = window.height() / 2.;
    let z_position: f32 = SPRITE_DEPTH;

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

    let health = Health::new(INITIAL_HEALTH, HEALTH_REGEN);

    let projectile_cast_timer = CastTimer {
        timer: Timer::new(Duration::from_secs_f32(1. / CAST_SPEED), TimerMode::Once),
    };

    let score = Score { score: 0 };

    commands.spawn((sprite_bundle, Player, health, projectile_cast_timer, score));
}

pub fn movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
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

        direction = direction
            .normalize_or_zero()
            .with_z(transform.translation.z);

        transform.translation += direction * SPEED * time.delta_seconds();
    }
}

pub fn restrict_movement(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    if let Ok(mut transform) = player_query.get_single_mut() {
        let radius = SPRITE_DIAMETER / 2.0;
        transform.translation = transform.translation.clamp(
            Vec3 {
                x: radius,
                y: radius,
                z: SPRITE_DEPTH,
            },
            Vec3 {
                x: window.width() - radius,
                y: window.height() - radius,
                z: SPRITE_DEPTH,
            },
        );
    }
}

pub fn check_dead(
    player_query: Query<&Health, With<Player>>,
    mut next_app_state: ResMut<NextState<AppState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    if let Ok(health) = player_query.get_single() {
        if health.is_dead() {
            println!("Player is dead");
            next_game_state.set(GameState::None);
            next_app_state.set(AppState::GameOver)
        }
    }
}

pub fn despawn(player_query: Query<Entity, With<Player>>, mut commands: Commands) {
    if let Ok(entity) = player_query.get_single() {
        commands.entity(entity).despawn();
    }
}

pub fn regen(mut player_query: Query<&mut Health, With<Player>>, time: Res<Time>) {
    if let Ok(mut health) = player_query.get_single_mut() {
        health.tick_regen(time.delta());
    }
}
