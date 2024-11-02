use bevy::prelude::{in_state, IntoSystemConfigs, OnExit, Plugin, Update};

mod components;
pub mod constants;
mod systems;

use systems::{despawn, hit_target, movement, spawn};

use crate::states::AppState;

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            Update,
            (spawn, movement, hit_target).run_if(in_state(AppState::Game)),
        )
        .add_systems(OnExit(AppState::Game), despawn);
    }
}
