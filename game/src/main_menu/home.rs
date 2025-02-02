use bevy::{
    app::Update,
    prelude::{in_state, App, IntoSystemConfigs, OnEnter, OnExit, Plugin},
};

use super::states::MainMenuState;

mod components;
mod constants;
mod systems;

use systems::{
    controls_button_interaction, despawn, esc_quit_game, quit_button_interaction, spawn,
    start_button_interaction,
};
pub struct HomePlugin;

impl Plugin for HomePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(MainMenuState::Home), spawn)
            .add_systems(OnExit(MainMenuState::Home), despawn)
            .add_systems(
                Update,
                (
                    start_button_interaction,
                    quit_button_interaction,
                    controls_button_interaction,
                    esc_quit_game,
                )
                    .run_if(in_state(MainMenuState::Home)),
            );
    }
}
