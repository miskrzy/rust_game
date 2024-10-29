use std::time::Duration;

use bevy::{
    asset::AssetServer,
    math::{
        bounding::{BoundingCircle, IntersectsVolume},
        Vec2, Vec3,
    },
    prelude::{Commands, Query, Res, Transform, With, Without},
    sprite::{Sprite, SpriteBundle},
    time::{Time, Timer, TimerMode},
    window::{PrimaryWindow, Window},
};
use rand::{thread_rng, Rng};

use crate::player::{
    components::{Health, Player},
    constants::SPRITE_DIAMETER as PLAYER_SPRITE_DIAMETER,
};

use super::components::{AttackTimer, Enemy};
use super::constants::{AMOUNT, ATTACK_SPEED, DAMAGE, SPEED, SPRITE_DIAMETER, TEXTURE_PATH};

pub fn spawn(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let mut rng_gen = thread_rng();

    for _ in 0..AMOUNT {
        let window = window_query.get_single().unwrap();
        let x_position = rng_gen.gen_range(0.0..window.width());
        let y_position = rng_gen.gen_range(0.0..window.height());
        let z_position: f32 = 0.0;

        let texture = asset_server.load(TEXTURE_PATH);

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

        let attack_timer = AttackTimer {
            timer: Timer::new(Duration::from_secs_f32(1. / ATTACK_SPEED), TimerMode::Once),
        };

        commands.spawn((sprite_bundle, Enemy, attack_timer));
    }
}

pub fn movement(
    mut enemy_query: Query<&mut Transform, (With<Enemy>, Without<Player>)>,
    player_query: Query<&Transform, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        for mut transform in enemy_query.iter_mut() {
            transform.translation = transform
                .translation
                .move_towards(player_transform.translation, SPEED * time.delta_seconds());
        }
    }
}

pub fn restrict_movement(
    mut enemy_query: Query<&mut Transform, With<Enemy>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    for mut transform in enemy_query.iter_mut() {
        let radius = SPRITE_DIAMETER / 2.;
        transform.translation = transform.translation.clamp(
            Vec3 {
                x: radius,
                y: radius,
                z: 0.0,
            },
            Vec3 {
                x: window.width() - radius,
                y: window.height() - radius,
                z: 0.0,
            },
        );
    }
}

pub fn attack_player(
    mut enemy_query: Query<(&Transform, &mut AttackTimer), (With<Enemy>, Without<Player>)>,
    mut player_query: Query<(&Transform, &mut Health), With<Player>>,
    time: Res<Time>,
) {
    if let Ok((player_transform, mut player_health)) = player_query.get_single_mut() {
        let player_collider = BoundingCircle::new(
            player_transform.translation.truncate(),
            PLAYER_SPRITE_DIAMETER / 2.,
        );

        for (transform, mut attack_timer) in &mut enemy_query {
            attack_timer.timer.tick(time.delta());

            let collider =
                BoundingCircle::new(transform.translation.truncate(), SPRITE_DIAMETER / 2.);
            if collider.intersects(&player_collider) && attack_timer.timer.finished() {
                player_health.deal_damage(DAMAGE);
                println!("Player received {DAMAGE} damage");
                attack_timer.timer.reset();
            }
        }
    }
}
