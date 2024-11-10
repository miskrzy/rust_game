use bevy::app::Update;
use bevy::prelude::{in_state, IntoSystemConfigs, OnEnter};
use bevy::prelude::{OnExit, Plugin};

mod components;
mod constants;
mod systems;

use systems::{despawn, spawn, update_health_bar, update_score};

use crate::states::AppState;

pub struct HUDPlugin;

impl Plugin for HUDPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(OnEnter(AppState::Game), spawn)
            .add_systems(
                Update,
                (update_health_bar, update_score).run_if(in_state(AppState::Game)),
            )
            .add_systems(OnExit(AppState::Game), despawn);
    }
}
