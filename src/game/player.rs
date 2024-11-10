use bevy::{
    app::Update,
    prelude::{in_state, App, Condition, IntoSystemConfigs, OnEnter, OnExit, Plugin},
};

pub mod components;
pub mod constants;
mod systems;

use systems::{despawn, despawn_dead, movement, regen, restrict_movement, spawn};

use crate::states::AppState;

use super::states::GameState;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Game), spawn)
            .add_systems(
                Update,
                ((movement, restrict_movement).chain(), (despawn_dead, regen))
                    .run_if(in_state(AppState::Game).and_then(in_state(GameState::Play))),
            )
            .add_systems(OnExit(AppState::Game), despawn);
    }
}
