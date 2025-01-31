use super::super::{
    enemies::{components::Enemy, constants::SPRITE_DIAMETER as ENEMY_SPRITE_DIAMETER},
    explosion::components::ShouldExplode,
    player::components::{CastTimer, Health, Player},
};
use super::constants::DAMAGE;
use super::{
    components::Projectile,
    constants::{PROJECTILE_TEXTURE_PATH, SPEED, SPRITE_DEPTH, SPRITE_DIAMETER},
};
use bevy::{
    asset::AssetServer,
    math::{
        bounding::{BoundingCircle, IntersectsVolume},
        Vec2, Vec3,
    },
    prelude::{Commands, Entity, Query, Res, Transform, With, Without},
    sprite::Sprite,
    time::Time,
};

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
                let start_position = player_transform.translation.with_z(SPRITE_DEPTH);
                let target_position = enemy_translation.with_z(SPRITE_DEPTH);
                let projectile = Projectile::new(start_position, target_position, SPEED);

                let sprite = Sprite {
                    image: asset_server.load(PROJECTILE_TEXTURE_PATH),
                    custom_size: Some(Vec2::new(SPRITE_DIAMETER, SPRITE_DIAMETER)),
                    ..Default::default()
                };

                let transform = Transform::from_translation(start_position);

                commands.spawn((projectile, transform, sprite));
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
    mut enemy_query: Query<(&Transform, &mut Health, Entity), With<Enemy>>,
    mut commands: Commands,
) {
    for (projectile, transform, entity) in projectile_query.iter() {
        if projectile.is_finished() {
            commands.entity(entity).despawn();
            continue;
        }
        for (enemy_transform, mut enemy_health, enemy_entity) in enemy_query.iter_mut() {
            let projectile_collider =
                BoundingCircle::new(transform.translation.truncate(), SPRITE_DIAMETER / 2.);
            let enemy_collider = BoundingCircle::new(
                enemy_transform.translation.truncate(),
                ENEMY_SPRITE_DIAMETER / 2.,
            );
            if projectile_collider.intersects(&enemy_collider) {
                commands.entity(entity).despawn();
                enemy_health.deal_damage(DAMAGE);
                if !enemy_health.is_dead() {
                    commands.entity(enemy_entity).insert(ShouldExplode);
                }
                break;
            }
        }
    }
}

pub fn despawn(projectile_query: Query<Entity, With<Projectile>>, mut commands: Commands) {
    for entity in projectile_query.iter() {
        commands.entity(entity).despawn();
    }
}
