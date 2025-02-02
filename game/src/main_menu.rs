use bevy::prelude::{App, AppExtStates, Plugin};

mod controls;
mod home;
pub mod states;

use controls::ControlsPlugin;
use home::HomePlugin;
use states::MainMenuState;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((ControlsPlugin, HomePlugin))
            .insert_state(MainMenuState::Home);
    }
}
