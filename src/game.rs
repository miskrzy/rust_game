use bevy::{
    app::{Startup, Update},
    prelude::{App, Plugin},
};

mod enemies;
mod hud;
mod others;
mod player;
mod projectiles;

use enemies::EnemyPlugin;
use hud::HUDPlugin;
use others::{exit_game, spawn_camera};
use player::PlayerPlugin;
use projectiles::ProjectilePlugin;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((PlayerPlugin, EnemyPlugin, HUDPlugin, ProjectilePlugin))
            .add_systems(Startup, spawn_camera)
            .add_systems(Update, exit_game);
    }
}
