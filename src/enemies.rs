use bevy::{
    prelude::{IntoSystemConfigs, Plugin, Startup, Update},
    time::{Timer, TimerMode},
};

pub mod components;
pub mod constants;
mod resources;
mod systems;
use constants::SPAWN_DELAY;
use resources::SpawnTimer;
use systems::{attack_player, initial_spawn, movement, restrict_movement, spawn_over_time};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(SpawnTimer {
            timer: Timer::from_seconds(SPAWN_DELAY, TimerMode::Repeating),
        })
        .add_systems(Startup, initial_spawn)
        .add_systems(Update, (movement, restrict_movement).chain())
        .add_systems(Update, (attack_player, spawn_over_time));
    }
}
