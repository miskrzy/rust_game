use bevy::prelude::{App, AppExtStates, Plugin};

mod arena;
mod camera;
mod enemies;
mod hud;
mod pause_menu;
mod player;
mod projectiles;
pub mod states;

use arena::ArenaPlugin;
use camera::CameraPlugin;
use enemies::EnemyPlugin;
use hud::HUDPlugin;
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
            CameraPlugin,
        ))
        .insert_state(GameState::None);
    }
}
