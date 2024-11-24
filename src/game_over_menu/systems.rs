use bevy::{
    color::{
        palettes::css::{BLACK, GRAY, WHITE},
        Color,
    },
    prelude::{
        BuildChildren, ButtonBundle, Changed, Commands, DespawnRecursiveExt, Entity, NextState,
        NodeBundle, Query, ResMut, TextBundle, With,
    },
    text::{Text, TextStyle},
    ui::{
        AlignItems, BackgroundColor, BorderColor, BorderRadius, Display, FlexDirection,
        Interaction, JustifyContent, PositionType, Style, UiRect, Val,
    },
};

use super::{
    components::{GameOverMenu, MenuButton, RestartButton},
    constants::{BUTTON_COLOR, BUTTON_HOVERED_COLOR},
};
use crate::states::AppState;
use crate::{game::states::GameState, main_menu::states::MainMenuState};

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
    let game_over_text = TextBundle {
        text: Text::from_section(
            "GAME OVER",
            TextStyle {
                color: Color::Srgba(WHITE),
                font_size: 60.,
                ..Default::default()
            },
        ),
        style: Style {
            margin: UiRect::all(Val::Px(10.)),
            ..Default::default()
        },
        ..Default::default()
    };
    let restart_button_node = ButtonBundle {
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
    let restart_text = TextBundle {
        text: Text::from_section(
            "Restart",
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
    let menu_button_node = ButtonBundle {
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
    let menu_text = TextBundle {
        text: Text::from_section(
            "Main menu",
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
        .spawn((screen_node, GameOverMenu))
        .with_children(|parent| {
            parent.spawn(game_over_text);
            parent
                .spawn((restart_button_node, RestartButton))
                .with_children(|parent: &mut bevy::prelude::ChildBuilder<'_>| {
                    parent.spawn(restart_text);
                });
            parent
                .spawn((menu_button_node, MenuButton))
                .with_children(|parent| {
                    parent.spawn(menu_text);
                });
        });
}

pub fn despawn(mut commands: Commands, pause_menu_query: Query<Entity, With<GameOverMenu>>) {
    if let Ok(entity) = pause_menu_query.get_single() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn restart_button_interaction(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (With<RestartButton>, Changed<Interaction>),
    >,
    mut next_app_state: ResMut<NextState<AppState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                next_game_state.set(GameState::Play);
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

pub fn menu_button_interaction(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (With<MenuButton>, Changed<Interaction>),
    >,
    mut next_app_state: ResMut<NextState<AppState>>,
    mut next_main_menu_state: ResMut<NextState<MainMenuState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                next_app_state.set(AppState::MainMenu);
                next_main_menu_state.set(MainMenuState::Home);
                next_game_state.set(GameState::None);
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
