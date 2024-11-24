use bevy::{
    app::Update,
    prelude::{in_state, App, IntoSystemConfigs, OnEnter, OnExit, Plugin},
};

pub mod components;
pub mod constants;
mod systems;

use super::states::GameState;
use crate::states::AppState;
use systems::{check_dead, despawn, movement, regen, restrict_movement, spawn};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Game), spawn)
            .add_systems(
                Update,
                ((movement, restrict_movement).chain(), (check_dead, regen))
                    .run_if(in_state(GameState::Play)),
            )
            .add_systems(OnExit(AppState::Game), despawn);
    }
}
