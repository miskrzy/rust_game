use bevy::{
    color::{
        palettes::css::{BLACK, GRAY, WHITE},
        Color,
    },
    prelude::{
        BuildChildren, Button, Changed, ChildBuild, Commands, DespawnRecursiveExt, Entity,
        NextState, Node, Query, ResMut, Text, With,
    },
    text::TextColor,
    ui::{
        AlignItems, BackgroundColor, BorderColor, BorderRadius, Display, FlexDirection,
        Interaction, JustifyContent, PositionType, UiRect, Val,
    },
};

use super::{
    components::{GameOverMenu, MenuButton, RestartButton},
    constants::{BUTTON_COLOR, BUTTON_HOVERED_COLOR},
};
use crate::states::AppState;
use crate::{game::states::GameState, main_menu::states::MainMenuState};

pub fn spawn(mut commands: Commands) {
    let screen_node_bundle = (
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
        GameOverMenu,
    );
    let game_over_text_node = Node {
        display: Display::Flex,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        margin: UiRect::all(Val::Px(10.)),
        ..Default::default()
    };
    let game_over_text_bundle = (
        Text::new("GAME OVER"),
        TextColor(Color::Srgba(WHITE)),
        // margin: UiRect::all(Val::Px(10.)),
    );
    let restart_button_bundle = (
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
        RestartButton,
    );
    let restart_text_node = Node {
        display: Display::Flex,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        margin: UiRect::all(Val::Px(10.)),
        ..Default::default()
    };
    let restart_text_bundle = (
        Text::new("Restart"),
        TextColor(Color::Srgba(WHITE)),
        // margin: UiRect::all(Val::Px(10.)),
    );
    let menu_button_bundle = (
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
        MenuButton,
    );
    let menu_text_node = Node {
        display: Display::Flex,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        margin: UiRect::all(Val::Px(10.)),
        ..Default::default()
    };
    let menu_text_bundle = (
        Text::new("Main menu"),
        TextColor(Color::Srgba(WHITE)),
        // margin: UiRect::all(Val::Px(10.)),
    );
    commands.spawn(screen_node_bundle).with_children(|parent| {
        parent
            .spawn(game_over_text_node)
            .with_child(game_over_text_bundle);
        parent.spawn(restart_button_bundle).with_children(|parent| {
            parent
                .spawn(restart_text_node)
                .with_child(restart_text_bundle);
        });
        parent.spawn(menu_button_bundle).with_children(|parent| {
            parent.spawn(menu_text_node).with_child(menu_text_bundle);
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
