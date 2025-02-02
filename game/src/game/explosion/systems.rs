use super::super::enemies::{components::Enemy, constants::SPRITE_DIAMETER as ENEMY_DIAMETER};
use super::super::player::components::Health;

use super::components::Explosion;
use super::constants::{
    DAMAGE, DIAMETER, DURATION, OPACITY, REPEAT, TEXTURE_COLUMNS, TEXTURE_PATH, TEXTURE_ROWS,
    TEXTURE_SIZE,
};
use super::events::Explode;
use super::resources::AssetHandles;
use bevy::color::Alpha;
use bevy::ecs::event::EventReader;
use bevy::ecs::query::With;
use bevy::ecs::system::ResMut;
use bevy::math::bounding::{BoundingCircle, IntersectsVolume};
use bevy::math::{UVec2, Vec2};
use bevy::sprite::{TextureAtlas, TextureAtlasLayout};
use bevy::time::Time;
use bevy::{
    asset::{AssetServer, Assets},
    prelude::{Commands, Entity, Query, Res, Transform},
    sprite::Sprite,
};

pub fn startup(mut asset_handles: ResMut<AssetHandles>, asset_server: Res<AssetServer>) {
    asset_handles.texture = asset_server.load(TEXTURE_PATH);
}

pub fn spawn(
    asset_handles: Res<AssetHandles>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut commands: Commands,
    mut explode_events: EventReader<Explode>,
) {
    let layout = TextureAtlasLayout::from_grid(
        UVec2::splat(TEXTURE_SIZE),
        TEXTURE_COLUMNS,
        TEXTURE_ROWS,
        None,
        None,
    );
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    let mut sprite = Sprite {
        image: asset_handles.texture.clone(),
        custom_size: Some(Vec2 {
            x: DIAMETER,
            y: DIAMETER,
        }),
        texture_atlas: Some(TextureAtlas {
            layout: texture_atlas_layout,
            index: 0,
        }),
        ..Default::default()
    };
    sprite.color.set_alpha(OPACITY);

    let explosion = Explosion::new(
        DURATION,
        usize::try_from(TEXTURE_COLUMNS * TEXTURE_ROWS).unwrap(),
        REPEAT,
    );

    for explode_event in explode_events.read() {
        commands.spawn((sprite.clone(), explode_event.transform, explosion.clone()));
    }
}

pub fn step_explosion(time: Res<Time>, mut explosion_query: Query<&mut Explosion>) {
    for mut explosion in explosion_query.iter_mut() {
        explosion.step(time.delta());
    }
}

pub fn animate(mut explosion_query: Query<(&mut Sprite, &Explosion)>) {
    for (mut sprite, explosion) in explosion_query.iter_mut() {
        if let Some(texture_atlas) = &mut sprite.texture_atlas {
            texture_atlas.index = explosion.current_frame()
        }
    }
}

pub fn hit_targets(
    explosion_query: Query<(&Explosion, &Transform)>,
    mut enemy_query: Query<(&mut Health, &Transform), With<Enemy>>,
) {
    for (explosion, transform) in explosion_query.iter() {
        if explosion.hit() {
            for (mut enemy_health, enemy_transform) in enemy_query.iter_mut() {
                let explosion_collider =
                    BoundingCircle::new(transform.translation.truncate(), DIAMETER / 2.);
                let enemy_collider = BoundingCircle::new(
                    enemy_transform.translation.truncate(),
                    ENEMY_DIAMETER / 2.,
                );

                if explosion_collider.intersects(&enemy_collider) {
                    enemy_health.deal_damage(DAMAGE);
                }
            }
        }
    }
}

pub fn despawn_on_finish(explosion_query: Query<(Entity, &Explosion)>, mut commands: Commands) {
    for (entity, explosion) in explosion_query.iter() {
        if explosion.finished() {
            commands.entity(entity).despawn();
        }
    }
}

pub fn despawn(explosion_query: Query<Entity, With<Explosion>>, mut commands: Commands) {
    for entity in explosion_query.iter() {
        commands.entity(entity).despawn();
    }
}
