use bevy::prelude::{App, AppExtStates, DefaultPlugins};

pub mod game;
mod main_menu;
mod states;

use game::GamePlugin;
use main_menu::MainMenuPlugin;
use states::AppState;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, GamePlugin, MainMenuPlugin))
        .insert_state(AppState::MainMenu)
        .run();
}
