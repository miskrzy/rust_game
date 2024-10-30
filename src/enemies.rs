use systems::{attack_player, movement, restrict_movement, spawn};

use bevy::prelude::{IntoSystemConfigs, Plugin, Startup, Update};

pub mod components;
pub mod constants;
mod systems;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, spawn)
            .add_systems(Update, (movement, restrict_movement).chain())
            .add_systems(Update, attack_player);
    }
}
