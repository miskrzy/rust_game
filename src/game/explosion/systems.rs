use super::components::Explosion;
use super::constants::{
    DIAMETER, DURATION, REPEAT, TEXTURE_COLUMNS, TEXTURE_PATH, TEXTURE_ROWS, TEXTURE_SIZE,
};
use bevy::ecs::query::With;
use bevy::ecs::system::ResMut;
use bevy::math::{UVec2, Vec2};
use bevy::sprite::{TextureAtlas, TextureAtlasLayout};
use bevy::time::Time;
use bevy::{
    asset::{AssetServer, Assets},
    prelude::{Commands, Entity, Query, Res, Transform},
    sprite::Sprite,
};

pub fn spawn(
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut commands: Commands,
) {
    // load the sprite sheet using the `AssetServer`
    let texture = asset_server.load(TEXTURE_PATH);

    let layout = TextureAtlasLayout::from_grid(
        UVec2::splat(TEXTURE_SIZE),
        TEXTURE_COLUMNS,
        TEXTURE_ROWS,
        None,
        None,
    );
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    commands.spawn((
        Sprite {
            image: texture.clone(),
            custom_size: Some(Vec2 {
                x: DIAMETER,
                y: DIAMETER,
            }),
            texture_atlas: Some(TextureAtlas {
                layout: texture_atlas_layout.clone(),
                index: 0,
            }),
            ..Default::default()
        },
        Transform::from_xyz(100., 100., 100.),
        Explosion::new(
            DURATION,
            usize::try_from(TEXTURE_COLUMNS * TEXTURE_ROWS).unwrap(),
            REPEAT,
        ),
    ));
}

pub fn animate(time: Res<Time>, mut explosion_query: Query<(&mut Sprite, &mut Explosion)>) {
    for (mut sprite, mut explosion) in explosion_query.iter_mut() {
        explosion.step(time.delta());
        if let Some(texture_atlas) = &mut sprite.texture_atlas {
            texture_atlas.index = explosion.current_frame()
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
