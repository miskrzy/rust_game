use bevy::color::{
    palettes::tailwind::{GRAY_500, GRAY_700},
    Color,
};

pub const BUTTON_COLOR: Color = Color::Srgba(GRAY_700);
pub const BUTTON_HOVERED_COLOR: Color = Color::Srgba(GRAY_500);
pub const CONTROLS: [(&str, &str); 3] = [
    ("move", "wasd/arrows"),
    ("pause/unpause", "esc"),
    ("quit game (from main menu)", "esc"),
];
