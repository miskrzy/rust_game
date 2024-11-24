use bevy::{
    app::Startup,
    prelude::{App, AppExtStates, Plugin},
};

mod arena;
mod enemies;
mod hud;
mod others;
mod pause_menu;
mod player;
mod projectiles;
pub mod states;

use arena::ArenaPlugin;
use enemies::EnemyPlugin;
use hud::HUDPlugin;
use others::spawn_camera;
use pause_menu::PauseMenuPlugin;
use player::PlayerPlugin;
use projectiles::ProjectilePlugin;
use states::GameState;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            PlayerPlugin,
            EnemyPlugin,
            HUDPlugin,
            ProjectilePlugin,
            PauseMenuPlugin,
            ArenaPlugin,
        ))
        .insert_state(GameState::None)
        .add_systems(Startup, spawn_camera);
    }
}
