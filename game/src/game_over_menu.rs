use bevy::{
    app::Update,
    prelude::{in_state, App, IntoSystemConfigs, OnEnter, OnExit, Plugin},
};

mod components;
mod constants;
mod systems;

use systems::{despawn, menu_button_interaction, restart_button_interaction, spawn};

use crate::states::AppState;

pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::GameOver), spawn)
            .add_systems(OnExit(AppState::GameOver), despawn)
            .add_systems(
                Update,
                (menu_button_interaction, restart_button_interaction)
                    .run_if(in_state(AppState::GameOver)),
            );
    }
}
