use super::constants::DAMAGE;
use super::{
    super::{
        enemies::{components::Enemy, constants::SPRITE_DIAMETER as ENEMY_SPRITE_DIAMETER},
        explosion::events::Explode,
        player::components::{CastTimer, Health, Player},
    },
    resources::AssetHandles,
};
use super::{
    components::Projectile,
    constants::{SPEED, SPRITE_DEPTH, SPRITE_DIAMETER, TEXTURE_PATH},
};
use bevy::{
    asset::AssetServer,
    ecs::message::MessageWriter,
    math::{
        bounding::{BoundingCircle, IntersectsVolume},
        Vec2, Vec3,
    },
    prelude::{Commands, Entity, Query, Res, ResMut, Transform, With, Without},
    sprite::Sprite,
    time::Time,
};

pub fn startup(mut asset_handles: ResMut<AssetHandles>, asset_server: Res<AssetServer>) {
    asset_handles.texture = asset_server.load(TEXTURE_PATH);
}

pub fn spawn(
    mut player_query: Query<(&mut CastTimer, &Transform), With<Player>>,
    enemy_query: Query<&Transform, (With<Enemy>, Without<Player>)>,
    time: Res<Time>,
    mut commands: Commands,
    asset_handles: Res<AssetHandles>,
) {
    if let Ok((mut cast_timer, player_transform)) = player_query.single_mut() {
        cast_timer.timer.tick(time.delta());
        if cast_timer.timer.is_finished() {
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
                    image: asset_handles.texture.clone(),
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
    mut enemy_query: Query<(&Transform, &mut Health), With<Enemy>>,
    mut explode_events: MessageWriter<Explode>,
    mut commands: Commands,
) {
    for (projectile, transform, entity) in projectile_query.iter() {
        if projectile.is_finish() {
            commands.entity(entity).despawn();
            continue;
        }
        for (enemy_transform, mut enemy_health) in enemy_query.iter_mut() {
            let projectile_collider =
                BoundingCircle::new(transform.translation.truncate(), SPRITE_DIAMETER / 2.);
            let enemy_collider = BoundingCircle::new(
                enemy_transform.translation.truncate(),
                ENEMY_SPRITE_DIAMETER / 2.,
            );
            if projectile_collider.intersects(&enemy_collider) {
                commands.entity(entity).despawn();
                enemy_health.deal_damage(DAMAGE);
                explode_events.write(Explode {
                    transform: enemy_transform.clone(),
                });
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
