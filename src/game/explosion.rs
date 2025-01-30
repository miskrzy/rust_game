use bevy::{
    app::Update,
    ecs::schedule::IntoSystemConfigs,
    prelude::{App, Plugin},
    state::{
        condition::in_state,
        state::{OnEnter, OnExit},
    },
};

mod components;
pub mod constants;
mod systems;

use systems::{animate, despawn, despawn_on_finish, spawn};

use crate::states::AppState;

use super::states::GameState;

pub struct ExplosionPlugin;

impl Plugin for ExplosionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Game), spawn)
            .add_systems(OnExit(AppState::Game), despawn)
            .add_systems(
                Update,
                (animate, despawn_on_finish).run_if(in_state(GameState::Play)),
            );
    }
}
