use bevy::{
    app::{Startup, Update},
    prelude::{IntoSystemConfigs, Plugin},
};

pub mod components;
mod constants;
mod systems;
use systems::{player_movement, restrict_player_movement, spawn_player};
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, (player_movement, restrict_player_movement).chain());
    }
}
