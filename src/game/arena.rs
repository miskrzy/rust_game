use bevy::prelude::{OnEnter, OnExit, Plugin};
use systems::{despawn, spawn};

use crate::states::AppState;

mod components;
mod constants;
mod systems;

pub struct ArenaPlugin;

impl Plugin for ArenaPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(OnEnter(AppState::Game), spawn)
            .add_systems(OnExit(AppState::Game), despawn);
    }
}
