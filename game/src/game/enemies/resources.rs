use bevy::prelude::{Handle, Image, Resource, Timer};

#[derive(Resource)]
pub struct SpawnTimer {
    pub timer: Timer,
}

#[derive(Resource, Default)]
pub struct AssetHandles {
    pub texture: Handle<Image>,
}
