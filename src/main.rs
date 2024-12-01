use bevy::{
    prelude::{App, AppExtStates, DefaultPlugins, PluginGroup},
    window::{Window, WindowMode, WindowPlugin},
};

pub mod game;
mod game_over_menu;
mod main_menu;
mod states;

use game::GamePlugin;
use game_over_menu::GameOverPlugin;
use main_menu::MainMenuPlugin;
use states::AppState;

fn main() {
    let window_plugin = WindowPlugin {
        primary_window: Some(Window {
            mode: WindowMode::BorderlessFullscreen,
            ..Default::default()
        }),
        ..Default::default()
    };

    App::new()
        .add_plugins((
            DefaultPlugins.set(window_plugin),
            GamePlugin,
            MainMenuPlugin,
            GameOverPlugin,
        ))
        .insert_state(AppState::MainMenu)
        .run();
}
