use bevy::{
    asset::AssetServer,
    ecs::system::ResMut,
    math::Vec2,
    prelude::{Commands, Entity, Query, Res, Sprite, Transform, Window, With},
    sprite::SpriteImageMode,
    window::PrimaryWindow,
};

use super::{
    components::Arena,
    constants::{HEIGHT, SPRITE_DEPTH, TEXTURE_PATH, WIDTH},
    resources::AssetHandles,
};

pub fn startup(mut asset_handles: ResMut<AssetHandles>, asset_server: Res<AssetServer>) {
    asset_handles.texture = asset_server.load(TEXTURE_PATH);
}

pub fn spawn(
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_handles: Res<AssetHandles>,
    mut commands: Commands,
) {
    let window = window_query.single().unwrap();
    let arena_position = window.size() / 2.;

    let sprite = Sprite {
        custom_size: Some(Vec2::new(WIDTH, HEIGHT)),
        image: asset_handles.texture.clone(),
        image_mode: SpriteImageMode::Tiled {
            tile_x: true,
            tile_y: true,
            stretch_value: 1.,
        },
        ..Default::default()
    };

    commands.spawn((
        sprite,
        Transform::from_xyz(arena_position.x, arena_position.y, SPRITE_DEPTH),
        Arena,
    ));
}

pub fn despawn(arena_query: Query<Entity, With<Arena>>, mut commands: Commands) {
    if let Ok(entity) = arena_query.single() {
        commands.entity(entity).despawn();
    }
}
