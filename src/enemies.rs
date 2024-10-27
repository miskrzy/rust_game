use systems::{attack_player, enemies_movement, restrict_enemy_movement, spawn_enemies};

use bevy::prelude::{IntoSystemConfigs, Plugin, Startup, Update};

mod components;
mod constants;
mod systems;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, spawn_enemies)
            .add_systems(Update, (enemies_movement, restrict_enemy_movement).chain())
            .add_systems(Update, attack_player);
    }
}
