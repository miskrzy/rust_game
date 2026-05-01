use bevy::prelude::Message;

#[derive(Message)]
pub struct FinalScore {
    pub score: u16,
}
