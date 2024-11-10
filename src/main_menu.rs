use bevy::{
    app::Update,
    prelude::{in_state, App, IntoSystemConfigs, OnEnter, OnExit, Plugin},
};

use crate::states::AppState;

mod components;
mod constants;
mod systems;

use systems::{despawn, spawn, start_button_interaction};

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::MainMenu), spawn)
            .add_systems(OnExit(AppState::MainMenu), despawn)
            .add_systems(
                Update,
                start_button_interaction.run_if(in_state(AppState::MainMenu)),
            );
    }
}
