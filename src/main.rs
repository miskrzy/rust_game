use bevy::prelude::{App, AppExtStates, DefaultPlugins};

pub mod game;
mod game_over_menu;
mod main_menu;
mod states;

use game::GamePlugin;
use game_over_menu::GameOverPlugin;
use main_menu::MainMenuPlugin;
use states::AppState;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, GamePlugin, MainMenuPlugin, GameOverPlugin))
        .insert_state(AppState::MainMenu)
        .run();
}
