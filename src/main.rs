use bevy::{
    app::Startup,
    prelude::{App, DefaultPlugins},
};

mod camera;
mod constants;
mod enemies;
mod player;

use camera::spawn_camera;
use enemies::EnemyPlugin;
use player::PlayerPlugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PlayerPlugin, EnemyPlugin))
        .add_systems(Startup, spawn_camera)
        .run();
}
