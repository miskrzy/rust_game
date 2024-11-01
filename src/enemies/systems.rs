use std::time::Duration;

use bevy::{
    asset::AssetServer,
    math::{
        bounding::{BoundingCircle, IntersectsVolume},
        Vec2, Vec3,
    },
    prelude::{Commands, Entity, Query, Res, ResMut, Transform, With, Without},
    sprite::{Sprite, SpriteBundle},
    time::{Time, Timer, TimerMode},
    window::{PrimaryWindow, Window},
};
use rand::{thread_rng, Rng};

use crate::player::{
    components::{Health, Player},
    constants::SPRITE_DIAMETER as PLAYER_SPRITE_DIAMETER,
};

use super::constants::{
    ATTACK_SPEED, DAMAGE, INITIAL_AMOUNT, INITIAL_HEALTH, SPAWN_AROUND_PLAYER_RADIUS, SPEED,
    SPRITE_DIAMETER, TEXTURE_PATH,
};
use super::{
    components::{AttackTimer, Enemy},
    resources::SpawnTimer,
};

fn create_random_position(
    screen_l: f32,
    screen_r: f32,
    screen_d: f32,
    screen_u: f32,
    player_x: f32,
    player_y: f32,
    player_r: f32,
    enemy_r: f32,
) -> (f32, f32) {
    if (screen_u - screen_d) <= (player_r + 2. * enemy_r) * 2. {
        panic!("spawn radius is too large for the height of the screen");
    }
    let mut rng_gen = thread_rng();
    let x = rng_gen.gen_range((screen_l + enemy_r)..(screen_r - enemy_r));
    if x < player_x + player_r + enemy_r && x > player_x - player_r - enemy_r {
        let offset = ((player_r + enemy_r).powi(2) - (x - player_x).powi(2)).sqrt();
        let mut y = rng_gen.gen_range((screen_d + enemy_r)..(screen_u - enemy_r - 2.0 * offset));
        y = if y > player_y - offset {
            y + 2. * offset
        } else {
            y
        };
        (x, y)
    } else {
        let y = rng_gen.gen_range(screen_d..screen_u);
        (x, y)
    }
}

fn create_entity_bundle(
    window_query: &Query<&Window, With<PrimaryWindow>>,
    asset_server: &Res<AssetServer>,
    player_query: &Query<&Transform, With<Player>>,
) -> (SpriteBundle, Enemy, AttackTimer, Health) {
    let window = window_query.get_single().unwrap();
    let window_width = window.width();
    let window_height = window.height();

    let (player_x, player_y) = if let Ok(player_transform) = player_query.get_single() {
        (
            player_transform.translation.x,
            player_transform.translation.y,
        )
    } else {
        (window_width / 2., window_height / 2.)
    };

    let enemy_radius = SPRITE_DIAMETER / 2.;

    let (x_position, y_position) = create_random_position(
        enemy_radius,
        window_width - enemy_radius,
        enemy_radius,
        window_height - enemy_radius,
        player_x,
        player_y,
        SPAWN_AROUND_PLAYER_RADIUS,
        SPRITE_DIAMETER / 2.,
    );
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

    let mut timer = Timer::new(Duration::from_secs_f32(1. / ATTACK_SPEED), TimerMode::Once);
    timer.tick(Duration::from_secs_f32(1. / ATTACK_SPEED));
    let attack_timer = AttackTimer { timer: timer };

    (
        sprite_bundle,
        Enemy,
        attack_timer,
        Health::new(INITIAL_HEALTH),
    )
}

pub fn initial_spawn(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    player_query: Query<&Transform, With<Player>>,
) {
    for _ in 0..INITIAL_AMOUNT {
        commands.spawn(create_entity_bundle(
            &window_query,
            &asset_server,
            &player_query,
        ));
    }
}

pub fn spawn_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    mut spawn_timer: ResMut<SpawnTimer>,
    time: Res<Time>,
    player_query: Query<&Transform, With<Player>>,
) {
    spawn_timer.timer.tick(time.delta());
    if spawn_timer.timer.finished() {
        commands.spawn(create_entity_bundle(
            &window_query,
            &asset_server,
            &player_query,
        ));
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

pub fn despawn(enemy_query: Query<(Entity, &Health), With<Enemy>>, mut commands: Commands) {
    for (entity, health) in enemy_query.iter() {
        if health.is_dead() {
            commands.entity(entity).despawn()
        }
    }
}
