use bevy::{
    prelude::{
        in_state, App, IntoScheduleConfigs, OnEnter, OnExit, Plugin, Startup, SystemCondition,
        Update,
    },
    time::{Timer, TimerMode},
};

pub mod components;
pub mod constants;
mod resources;
mod systems;

use constants::SPAWN_DELAY;
use resources::{AssetHandles, SpawnTimer};
use systems::{
    attack_player, despawn, despawn_dead, initial_spawn, movement, restrict_movement,
    spawn_over_time, startup,
};

use crate::states::AppState;

use super::states::GameState;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpawnTimer {
            timer: Timer::from_seconds(SPAWN_DELAY, TimerMode::Repeating),
        })
        .init_resource::<AssetHandles>()
        .add_systems(Startup, startup)
        .add_systems(OnEnter(AppState::Game), initial_spawn)
        .add_systems(
            Update,
            (
                (movement, restrict_movement).chain(),
                (attack_player, spawn_over_time, despawn_dead),
            )
                .run_if(in_state(AppState::Game).and(in_state(GameState::Play))),
        )
        .add_systems(OnExit(AppState::Game), despawn);
    }
}
