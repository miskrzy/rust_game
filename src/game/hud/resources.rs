use bevy::prelude::Resource;

#[derive(Resource)]
pub struct Score {
    pub score: u16,
}
