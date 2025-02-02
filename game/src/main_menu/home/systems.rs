use super::super::states::MainMenuState;
use super::components::{ControlsButton, MainMenu, QuitButton, StartButton};
use super::constants::{BUTTON_COLOR, BUTTON_HOVERED_COLOR};
use crate::game::states::GameState;
use crate::states::AppState;
use bevy::prelude::ChildBuild;
use bevy::text::TextColor;
use bevy::{
    app::AppExit,
    color::{
        palettes::css::{BLACK, GRAY, WHITE},
        Color,
    },
    input::ButtonInput,
    prelude::{
        BuildChildren, Button, Changed, Commands, DespawnRecursiveExt, Entity, EventWriter,
        KeyCode, NextState, Node, Query, Res, ResMut, Text, With,
    },
    ui::{
        AlignItems, BackgroundColor, BorderColor, BorderRadius, Display, FlexDirection,
        Interaction, JustifyContent, PositionType, UiRect, Val,
    },
};

pub fn spawn(mut commands: Commands) {
    let screen_bundle = (
        Node {
            display: Display::Flex,
            position_type: PositionType::Absolute,
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        BackgroundColor(Color::Srgba(GRAY)),
        MainMenu,
    );
    let start_button_bundle = (
        Node {
            display: Display::Flex,
            border: UiRect::all(Val::Px(2.)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            margin: UiRect::all(Val::Px(10.)),
            ..Default::default()
        },
        BackgroundColor(BUTTON_COLOR),
        BorderColor(Color::Srgba(BLACK)),
        BorderRadius::all(Val::Percent(50.)),
        Button,
        StartButton,
    );
    let start_text_node = Node {
        display: Display::Flex,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        margin: UiRect::all(Val::Px(10.)),
        ..Default::default()
    };
    let start_text_bundle = (Text::new("Start Game"), TextColor(Color::Srgba(WHITE)));
    let quit_button_bundle = (
        Node {
            display: Display::Flex,
            border: UiRect::all(Val::Px(2.)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            margin: UiRect::vertical(Val::Px(10.)),
            ..Default::default()
        },
        Button,
        BackgroundColor(BUTTON_COLOR),
        BorderColor(Color::Srgba(BLACK)),
        BorderRadius::all(Val::Percent(50.)),
        QuitButton,
    );
    let quit_text_node = Node {
        display: Display::Flex,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        margin: UiRect::all(Val::Px(10.)),
        ..Default::default()
    };
    let quit_text_bundle = (Text::new("Quit Game"), TextColor(Color::Srgba(WHITE)));
    let controls_button_bundle = (
        Node {
            display: Display::Flex,
            border: UiRect::all(Val::Px(2.)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            margin: UiRect::vertical(Val::Px(10.)),
            ..Default::default()
        },
        BackgroundColor(BUTTON_COLOR),
        BorderColor(Color::Srgba(BLACK)),
        BorderRadius::all(Val::Percent(50.)),
        Button,
        ControlsButton,
    );
    let controls_text_node = Node {
        display: Display::Flex,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        margin: UiRect::all(Val::Px(10.)),
        ..Default::default()
    };
    let controls_text_bundle = (Text::new("Controls"), TextColor(Color::Srgba(WHITE)));
    commands.spawn(screen_bundle).with_children(|parent| {
        parent.spawn(start_button_bundle).with_children(|parent| {
            parent.spawn(start_text_node).with_child(start_text_bundle);
        });
        parent.spawn(quit_button_bundle).with_children(|parent| {
            parent.spawn(quit_text_node).with_child(quit_text_bundle);
        });
        parent
            .spawn(controls_button_bundle)
            .with_children(|parent| {
                parent
                    .spawn(controls_text_node)
                    .with_child(controls_text_bundle);
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
    mut next_game_state: ResMut<NextState<GameState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
    mut next_main_menu_state: ResMut<NextState<MainMenuState>>,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                next_app_state.set(AppState::Game);
                next_game_state.set(GameState::Play);
                next_main_menu_state.set(MainMenuState::None)
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

pub fn controls_button_interaction(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (With<ControlsButton>, Changed<Interaction>),
    >,
    mut next_main_menu_state: ResMut<NextState<MainMenuState>>,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                next_main_menu_state.set(MainMenuState::Controls);
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
