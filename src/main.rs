use bevy::{
    app::Update,
    prelude::{App, AppExtStates, DefaultPlugins},
};

pub mod game;
mod main_menu;
mod states;
mod systems;

use game::GamePlugin;
use main_menu::MainMenuPlugin;
use states::AppState;
use systems::change_app_state;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, GamePlugin, MainMenuPlugin))
        .insert_state(AppState::MainMenu)
        .add_systems(Update, change_app_state)
        .run();
}
