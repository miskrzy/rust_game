use bevy::{
    app::{Startup, Update},
    prelude::{in_state, IntoSystemConfigs, Plugin},
};
use systems::{follow_player, spawn};

use super::states::GameState;

mod components;
mod systems;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, spawn)
            .add_systems(Update, follow_player.run_if(in_state(GameState::Play)));
    }
}
