use crate::game::player::components::Score;

use super::constants::DAMAGE;
use super::{
    super::{
        enemies::{components::Enemy, constants::SPRITE_DIAMETER as ENEMY_SPRITE_DIAMETER},
        player::components::{CastTimer, Health, Player},
    },
    constants::{
        EXPLOSION_TEXTURE_COLUMNS, EXPLOSION_TEXTURE_PATH, EXPLOSION_TEXTURE_ROWS,
        EXPLOSION_TEXTURE_SIZE,
    },
};
use super::{
    components::Projectile,
    constants::{PROJECTILE_TEXTURE_PATH, SPEED, SPRITE_DEPTH, SPRITE_DIAMETER},
};
use bevy::ecs::system::ResMut;
use bevy::sprite::{TextureAtlas, TextureAtlasLayout};
use bevy::{
    asset::{AssetServer, Assets},
    math::{
        bounding::{BoundingCircle, IntersectsVolume},
        UVec2, Vec2, Vec3,
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
    mut enemy_query: Query<(&Transform, &mut Health), With<Enemy>>,
    mut commands: Commands,
    mut player_query: Query<&mut Score, With<Player>>,
) {
    for (projectile, transform, entity) in projectile_query.iter() {
        if projectile.is_finished() {
            commands.entity(entity).despawn();
            continue;
        }
        for (enemy_transform, mut health) in enemy_query.iter_mut() {
            let projectile_collider =
                BoundingCircle::new(transform.translation.truncate(), SPRITE_DIAMETER / 2.);
            let enemy_collider = BoundingCircle::new(
                enemy_transform.translation.truncate(),
                ENEMY_SPRITE_DIAMETER / 2.,
            );
            if projectile_collider.intersects(&enemy_collider) {
                commands.entity(entity).despawn();
                health.deal_damage(DAMAGE);
                if let Ok(mut score) = player_query.get_single_mut() {
                    score.score += 1;
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

pub fn spawn_explosion(
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut commands: Commands,
) {
    // load the sprite sheet using the `AssetServer`
    let texture = asset_server.load(EXPLOSION_TEXTURE_PATH);

    let layout = TextureAtlasLayout::from_grid(
        UVec2::splat(EXPLOSION_TEXTURE_SIZE),
        EXPLOSION_TEXTURE_COLUMNS,
        EXPLOSION_TEXTURE_ROWS,
        None,
        None,
    );
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    commands.spawn((
        Sprite {
            image: texture.clone(),
            // custom_size: Some(Vec2::new(EXPLOSION_TEXTURE_SIZE, EXPLOSION_TEXTURE_SIZE)),
            texture_atlas: Some(TextureAtlas {
                layout: texture_atlas_layout.clone(),
                index: 0,
            }),
            ..Default::default()
        },
        Transform::from_xyz(100., 100., 100.),
    ));
}
