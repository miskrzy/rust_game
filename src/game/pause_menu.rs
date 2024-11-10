use bevy::{
    app::Update,
    prelude::{in_state, App, IntoSystemConfigs, OnEnter, OnExit, Plugin},
};

mod components;
mod constants;
mod systems;

use systems::{
    change_game_state, despawn, quit_button_interaction, resume_button_interaction, spawn,
};

use crate::states::AppState;

use super::states::GameState;

pub struct PauseMenuPlugin;

impl Plugin for PauseMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Pause), spawn)
            .add_systems(OnExit(GameState::Pause), despawn)
            .add_systems(Update, change_game_state.run_if(in_state(AppState::Game)))
            .add_systems(
                Update,
                (quit_button_interaction, resume_button_interaction)
                    .run_if(in_state(GameState::Pause)),
            );
    }
}
