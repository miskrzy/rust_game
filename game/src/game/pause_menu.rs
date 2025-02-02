use bevy::{
    app::Update,
    prelude::{in_state, App, IntoSystemConfigs, OnEnter, OnExit, Plugin},
};

mod components;
mod constants;
mod systems;

use systems::{
    despawn, menu_button_interaction, pause_unpause_game, resume_button_interaction, spawn,
};

use super::states::GameState;
use crate::states::AppState;

pub struct PauseMenuPlugin;

impl Plugin for PauseMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Pause), spawn)
            .add_systems(OnExit(GameState::Pause), despawn)
            .add_systems(Update, pause_unpause_game.run_if(in_state(AppState::Game)))
            .add_systems(
                Update,
                (menu_button_interaction, resume_button_interaction)
                    .run_if(in_state(GameState::Pause)),
            );
    }
}
