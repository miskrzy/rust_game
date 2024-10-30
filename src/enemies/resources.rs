use bevy::prelude::*;

use super::constants::SPAWN_DELAY;

#[derive(Resource)]
pub struct SpawnTimer {
    pub timer: Timer,
}

impl Default for SpawnTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(SPAWN_DELAY, TimerMode::Repeating),
        }
    }
}
