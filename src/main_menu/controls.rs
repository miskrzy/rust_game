use bevy::{
    app::{App, Plugin, Update},
    prelude::{in_state, IntoSystemConfigs, OnEnter, OnExit},
};

mod components;
mod constants;
mod systems;

use super::states::MainMenuState;
use systems::{despawn, return_button_interaction, spawn};

pub struct ControlsPlugin;

impl Plugin for ControlsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(MainMenuState::Controls), spawn)
            .add_systems(OnExit(MainMenuState::Controls), despawn)
            .add_systems(
                Update,
                return_button_interaction.run_if(in_state(MainMenuState::Controls)),
            );
    }
}
