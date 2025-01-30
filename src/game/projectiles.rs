use bevy::prelude::{in_state, App, IntoSystemConfigs, OnExit, Plugin, Update};

mod components;
pub mod constants;
mod systems;

use systems::{despawn, hit_target, movement, spawn};

use super::states::GameState;
use crate::states::AppState;

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (spawn, movement, hit_target).run_if(in_state(GameState::Play)),
        )
        .add_systems(OnExit(AppState::Game), despawn);
    }
}
