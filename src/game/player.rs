use bevy::{
    app::Update,
    prelude::{in_state, App, IntoSystemConfigs, OnEnter, OnExit, Plugin, Startup},
};
use resources::AssetHandles;

pub mod components;
pub mod constants;
mod resources;
mod systems;

use super::states::GameState;
use crate::states::AppState;
use systems::{check_dead, despawn, movement, regen, restrict_movement, spawn, startup};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AssetHandles>()
            .add_systems(Startup, startup)
            .add_systems(OnEnter(AppState::Game), spawn)
            .add_systems(
                Update,
                ((movement, restrict_movement).chain(), (check_dead, regen))
                    .run_if(in_state(GameState::Play)),
            )
            .add_systems(OnExit(AppState::Game), despawn);
    }
}
