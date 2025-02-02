use bevy::prelude::{Handle, Image, Resource};

#[derive(Resource, Default)]
pub struct AssetHandles {
    pub texture: Handle<Image>,
}
