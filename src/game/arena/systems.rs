use bevy::{
    asset::AssetServer,
    math::Vec2,
    prelude::{Commands, Entity, Query, Res, Sprite, SpriteBundle, Transform, Window, With},
    sprite::ImageScaleMode,
    window::PrimaryWindow,
};

use super::components::Arena;
use super::constants::{HEIGHT, SPRITE_DEPTH, TEXTURE_PATH, WIDTH};

pub fn spawn(
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    let window = window_query.get_single().unwrap();
    let arena_position = window.size() / 2.;

    let sprite = Sprite {
        custom_size: Some(Vec2::new(WIDTH, HEIGHT)),
        ..Default::default()
    };

    let texture = asset_server.load(TEXTURE_PATH);

    let sprite_bundle = SpriteBundle {
        sprite: sprite,
        texture: texture,
        transform: Transform::from_xyz(arena_position.x, arena_position.y, SPRITE_DEPTH),
        ..Default::default()
    };

    commands.spawn((
        sprite_bundle,
        ImageScaleMode::Tiled {
            tile_x: true,
            tile_y: true,
            stretch_value: 0.5,
        },
        Arena,
    ));
}

pub fn despawn(arena_query: Query<Entity, With<Arena>>, mut commands: Commands) {
    if let Ok(entity) = arena_query.get_single() {
        commands.entity(entity).despawn();
    }
}
