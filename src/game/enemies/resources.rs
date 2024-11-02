use bevy::prelude::{Resource, Timer};

#[derive(Resource)]
pub struct SpawnTimer {
    pub timer: Timer,
}
