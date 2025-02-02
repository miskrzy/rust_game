use bevy::prelude::{OnEnter, OnExit, Plugin, Startup};

use crate::states::AppState;

mod components;
pub mod constants;
mod resources;
mod systems;

use resources::AssetHandles;
use systems::{despawn, spawn, startup};

pub struct ArenaPlugin;

impl Plugin for ArenaPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<AssetHandles>()
            .add_systems(Startup, startup)
            .add_systems(OnEnter(AppState::Game), spawn)
            .add_systems(OnExit(AppState::Game), despawn);
    }
}
