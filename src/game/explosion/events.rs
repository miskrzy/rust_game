use bevy::prelude::{Event, Transform};

#[derive(Event)]
pub struct Explode {
    pub transform: Transform,
}
