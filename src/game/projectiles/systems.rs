use crate::game::player::components::Score;

use super::super::{
    enemies::{components::Enemy, constants::SPRITE_DIAMETER as ENEMY_SPRITE_DIAMETER},
    player::components::{CastTimer, Health, Player},
};
use super::constants::DAMAGE;
use super::{
    components::Projectile,
    constants::{SPEED, SPRITE_DIAMETER, TEXTURE_PATH},
};
use bevy::{
    asset::AssetServer,
    math::{
        bounding::{BoundingCircle, IntersectsVolume},
        Vec2, Vec3,
    },
    prelude::{Commands, Entity, Query, Res, Transform, With, Without},
    sprite::{Sprite, SpriteBundle},
    time::Time,
};
use std::collections::HashSet;

pub fn spawn(
    mut player_query: Query<(&mut CastTimer, &Transform), With<Player>>,
    enemy_query: Query<&Transform, (With<Enemy>, Without<Player>)>,
    time: Res<Time>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    if let Ok((mut cast_timer, player_transform)) = player_query.get_single_mut() {
        cast_timer.timer.tick(time.delta());
        if cast_timer.timer.finished() {
            let mut closest_enemy: (Option<Vec3>, f32) = (None, f32::INFINITY);
            for enemy_transform in enemy_query.iter() {
                let player_enemy_distance = player_transform
                    .translation
                    .distance(enemy_transform.translation);
                if player_enemy_distance < closest_enemy.1 {
                    closest_enemy = (Some(enemy_transform.translation), player_enemy_distance);
                }
            }

            if let Some(enemy_translation) = closest_enemy.0 {
                let projectile =
                    Projectile::new(player_transform.translation, enemy_translation, SPEED);

                let sprite = Sprite {
                    custom_size: Some(Vec2::new(SPRITE_DIAMETER, SPRITE_DIAMETER)),
                    ..Default::default()
                };

                let sprite_bundle = SpriteBundle {
                    sprite: sprite,
                    texture: asset_server.load(TEXTURE_PATH),
                    transform: player_transform.clone(),
                    ..Default::default()
                };

                commands.spawn((projectile, sprite_bundle));
                cast_timer.timer.reset();
            }
        }
    }
}

pub fn movement(mut projectile_query: Query<(&mut Projectile, &mut Transform)>, time: Res<Time>) {
    for (mut projectile, mut transform) in projectile_query.iter_mut() {
        transform.translation += projectile.step(time.delta());
    }
}

pub fn hit_target(
    projectile_query: Query<(&Projectile, &Transform, Entity)>,
    mut enemy_query: Query<(&Transform, &mut Health), With<Enemy>>,
    mut commands: Commands,
    mut player_query: Query<&mut Score, With<Player>>,
) {
    let mut entities_to_despawn = HashSet::new();
    for (projectile, transform, entity) in projectile_query.iter() {
        let mut entity_exists = true;
        for (enemy_transform, mut health) in enemy_query.iter_mut() {
            let projectile_collider =
                BoundingCircle::new(transform.translation.truncate(), SPRITE_DIAMETER / 2.);
            let enemy_collider = BoundingCircle::new(
                enemy_transform.translation.truncate(),
                ENEMY_SPRITE_DIAMETER / 2.,
            );
            if projectile_collider.intersects(&enemy_collider) {
                entities_to_despawn.insert(entity);
                health.deal_damage(DAMAGE);
                entity_exists = false;
                if let Ok(mut score) = player_query.get_single_mut() {
                    score.score += 1;
                }
            }
        }
        if projectile.is_finished() && entity_exists {
            entities_to_despawn.insert(entity);
        }
    }
    for entity in entities_to_despawn {
        commands.entity(entity).despawn();
    }
}

pub fn despawn(projectile_query: Query<Entity, With<Projectile>>, mut commands: Commands) {
    for entity in projectile_query.iter() {
        commands.entity(entity).despawn();
    }
}
