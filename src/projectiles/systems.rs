use bevy::{
    asset::AssetServer,
    math::{Vec2, Vec3},
    prelude::{Commands, Query, Res, Transform, With},
    sprite::{Sprite, SpriteBundle},
    time::Time,
};

use crate::player::components::{CastTimer, Player};

use super::{
    components::Projectile,
    constants::{SPEED, SPRITE_DIAMETER, TEXTURE_PATH},
};

pub fn spawn(
    mut player_query: Query<(&mut CastTimer, &Transform), With<Player>>,
    time: Res<Time>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    if let Ok((mut cast_timer, player_transform)) = player_query.get_single_mut() {
        cast_timer.timer.tick(time.delta());
        if cast_timer.timer.finished() {
            let projectile = Projectile::init_with_target(Vec3::ZERO, SPEED);

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

pub fn movement(mut projectile_query: Query<(&Projectile, &mut Transform)>, time: Res<Time>) {
    for (projectile, mut transform) in projectile_query.iter_mut() {
        transform.translation =
            projectile.new_position(transform.translation, time.delta_seconds());
    }
}
