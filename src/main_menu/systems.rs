use bevy::{
    app::AppExit, color::{
        palettes::css::{BLACK, GRAY, WHITE},
        Color,
    }, input::ButtonInput, prelude::{
        BuildChildren, ButtonBundle, Changed, Commands, DespawnRecursiveExt, Entity, EventWriter, KeyCode, NextState, NodeBundle, Query, Res, ResMut, TextBundle, With
    }, text::{Text, TextStyle}, ui::{
        AlignItems, BackgroundColor, BorderColor, BorderRadius, Display, FlexDirection,
        Interaction, JustifyContent, PositionType, Style, UiRect, Val,
    }
};

use crate::states::AppState;

use super::components::{MainMenu, QuitButton, StartButton};
use super::constants::{BUTTON_COLOR, BUTTON_HOVERED_COLOR};

pub fn spawn(mut commands: Commands) {
    let screen_node = NodeBundle {
        style: Style {
            display: Display::Flex,
            position_type: PositionType::Absolute,
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            flex_direction: FlexDirection::Column,
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
        background_color: BackgroundColor(BUTTON_COLOR),
        border_color: BorderColor(Color::Srgba(BLACK)),
        border_radius: BorderRadius::all(Val::Percent(50.)),
        ..Default::default()
    };
    let start_text = TextBundle {
        text: Text::from_section(
            "Start Game",
            TextStyle {
                color: Color::Srgba(WHITE),
                ..Default::default()
            },
        ),
        style: Style {
            margin: UiRect::all(Val::Px(10.)),
            ..Default::default()
        },
        ..Default::default()
    };
    let quit_button_node = ButtonBundle {
        style: Style {
            display: Display::Flex,
            border: UiRect::all(Val::Px(2.)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            margin: UiRect::vertical(Val::Px(10.)),
            ..Default::default()
        },
        background_color: BackgroundColor(BUTTON_COLOR),
        border_color: BorderColor(Color::Srgba(BLACK)),
        border_radius: BorderRadius::all(Val::Percent(50.)),
        ..Default::default()
    };
    let quit_text = TextBundle {
        text: Text::from_section(
            "Quit Game",
            TextStyle {
                color: Color::Srgba(WHITE),
                ..Default::default()
            },
        ),
        style: Style {
            margin: UiRect::all(Val::Px(10.)),
            ..Default::default()
        },
        ..Default::default()
    };
    commands
        .spawn((screen_node, MainMenu))
        .with_children(|parent| {
            parent
                .spawn((start_button_node, StartButton))
                .with_children(|parent| {
                    parent.spawn(start_text);
                });
            parent
                .spawn((quit_button_node, QuitButton))
                .with_children(|parent| {
                    parent.spawn(quit_text);
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
                background_color.0 = BUTTON_HOVERED_COLOR;
            }
            Interaction::None => {
                background_color.0 = BUTTON_COLOR;
            }
        }
    }
}

pub fn quit_button_interaction(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (With<QuitButton>, Changed<Interaction>),
    >,
    mut exit: EventWriter<AppExit>,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                exit.send(AppExit::Success);
            }
            Interaction::Hovered => {
                background_color.0 = BUTTON_HOVERED_COLOR;
            }
            Interaction::None => {
                background_color.0 = BUTTON_COLOR;
            }
        }
    }
}

pub fn esc_quit_game(keyboard_input: Res<ButtonInput<KeyCode>>, mut exit: EventWriter<AppExit>) {
    if keyboard_input.pressed(KeyCode::Escape) {
        exit.send(AppExit::Success);
    }
}