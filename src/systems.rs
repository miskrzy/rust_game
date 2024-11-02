use bevy::{
    input::ButtonInput,
    prelude::{KeyCode, NextState, Res, ResMut, State},
};

use super::states::AppState;

pub fn change_app_state(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut next_app_state: ResMut<NextState<AppState>>,
    app_state: ResMut<State<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyM) {
        match *app_state.get() {
            AppState::Game => next_app_state.set(AppState::MainMenu),
            AppState::MainMenu => next_app_state.set(AppState::Game),
        }
    }
}
