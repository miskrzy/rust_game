use bevy::{
    app::{Startup, Update},
    prelude::{App, DefaultPlugins},
};

mod enemies;
mod others;
mod player;

use enemies::EnemyPlugin;
use others::{exit_game, spawn_camera};
use player::PlayerPlugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PlayerPlugin, EnemyPlugin))
        .add_systems(Startup, spawn_camera)
        .add_systems(Update, exit_game)
        .run();
}
