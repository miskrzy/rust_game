use bevy::prelude::{in_state, App, IntoScheduleConfigs, OnExit, Plugin, Startup, Update};

mod components;
pub mod constants;
mod resources;
mod systems;

use resources::AssetHandles;
use systems::{despawn, hit_target, movement, spawn, startup};

use super::states::GameState;
use crate::states::AppState;

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AssetHandles>()
            .add_systems(Startup, startup)
            .add_systems(
                Update,
                (spawn, movement, hit_target).run_if(in_state(GameState::Play)),
            )
            .add_systems(OnExit(AppState::Game), despawn);
    }
}
