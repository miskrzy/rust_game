use bevy::{
    app::{Startup, Update},
    prelude::{App, DefaultPlugins},
};

mod camera;
mod constants;
mod enemies;
mod player;

use camera::spawn_camera;
use enemies::{enemies_movement, spawn_enemies};
use player::{player_movement, spawn_player};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (spawn_player, spawn_camera, spawn_enemies))
        .add_systems(Update, (player_movement, enemies_movement))
        .run();
}
