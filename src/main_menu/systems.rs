use bevy::{
    color::{
        palettes::css::{BLACK, GRAY, WHITE},
        Color,
    },
    prelude::{
        BuildChildren, ButtonBundle, Changed, Commands, DespawnRecursiveExt, Entity, NextState,
        NodeBundle, Query, ResMut, TextBundle, With,
    },
    text::TextStyle,
    ui::{
        AlignItems, BackgroundColor, BorderColor, BorderRadius, Display, Interaction,
        JustifyContent, PositionType, Style, UiRect, Val,
    },
};

use crate::states::AppState;

use super::components::{MainMenu, StartButton};
use super::constants::{START_BUTTON_COLOR, START_BUTTON_HOVERED_COLOR};

pub fn spawn(mut commands: Commands) {
    let screen_node = NodeBundle {
        style: Style {
            display: Display::Flex,
            position_type: PositionType::Absolute,
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        background_color: BackgroundColor(Color::Srgba(GRAY)),
        ..Default::default()
    };
    let start_button_node = ButtonBundle {
        style: Style {
            display: Display::Flex,
            border: UiRect::all(Val::Px(2.)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        background_color: BackgroundColor(START_BUTTON_COLOR),
        border_color: BorderColor(Color::Srgba(BLACK)),
        border_radius: BorderRadius::all(Val::Percent(50.)),
        ..Default::default()
    };
    let start_text = TextBundle::from_section(
        " \n  Start Game  \n ",
        TextStyle {
            color: Color::Srgba(WHITE),
            ..Default::default()
        },
    );
    commands
        .spawn((screen_node, MainMenu))
        .with_children(|parent| {
            parent
                .spawn((start_button_node, StartButton))
                .with_children(|parent| {
                    parent.spawn(start_text);
                });
        });
}

pub fn despawn(mut commands: Commands, main_menu_query: Query<Entity, With<MainMenu>>) {
    if let Ok(entity) = main_menu_query.get_single() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn start_button_interaction(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (With<StartButton>, Changed<Interaction>),
    >,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                next_app_state.set(AppState::Game);
            }
            Interaction::Hovered => {
                background_color.0 = START_BUTTON_HOVERED_COLOR;
            }
            Interaction::None => {
                background_color.0 = START_BUTTON_COLOR;
            }
        }
    }
}
