use bevy::prelude::{Message, Transform};

#[derive(Message)]
pub struct Explode {
    pub transform: Transform,
}
