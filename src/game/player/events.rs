use bevy::prelude::Event;

#[derive(Event)]
pub struct FinalScore {
    pub score: u16,
}
