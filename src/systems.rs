use bevy::{
    input::ButtonInput,
    prelude::{KeyCode, NextState, Res, ResMut, State},
};

use crate::game::states::GameState;

use super::states::AppState;

pub fn change_app_state(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut next_app_state: ResMut<NextState<AppState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
    app_state: ResMut<State<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyM) {
        match *app_state.get() {
            AppState::Game => {
                next_app_state.set(AppState::MainMenu);
                next_game_state.set(GameState::Play);
            }
            AppState::MainMenu => {
                next_app_state.set(AppState::Game);
                next_game_state.set(GameState::Play);
            }
        }
    }
}
