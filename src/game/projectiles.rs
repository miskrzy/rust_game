use bevy::prelude::{in_state, App, Condition, IntoSystemConfigs, OnExit, Plugin, Update};

mod components;
pub mod constants;
mod systems;

use systems::{despawn, hit_target, movement, spawn};

use crate::states::AppState;

use super::states::GameState;

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (spawn, movement, hit_target)
                .run_if(in_state(AppState::Game).and_then(in_state(GameState::Play))),
        )
        .add_systems(OnExit(AppState::Game), despawn);
    }
}
