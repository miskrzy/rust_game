use bevy::{
    color::{
        palettes::css::{BLACK, GRAY, WHITE},
        Color,
    },
    ecs::message::MessageReader,
    prelude::{Button, Changed, Commands, Entity, NextState, Node, Query, ResMut, Text, With},
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
use crate::game::player::events::FinalScore;
use crate::states::AppState;
use crate::{game::states::GameState, main_menu::states::MainMenuState};

pub fn spawn(mut commands: Commands, mut final_score_events: MessageReader<FinalScore>) {
    let final_score = if let Some(final_score) = final_score_events.read().last() {
        final_score.score.to_string()
    } else {
        "N/A".to_string()
    };

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
        flex_direction: FlexDirection::Column,
        margin: UiRect::all(Val::Px(10.)),
        ..Default::default()
    };
    let game_over_text_bundle = (Text::new("GAME OVER"), TextColor(Color::Srgba(WHITE)));
    let final_score_text_bundle = (
        Text::new(format!("Final score: {}", final_score)),
        TextColor(Color::Srgba(WHITE)),
    );
    let restart_button_bundle = (
        Node {
            display: Display::Flex,
            border: UiRect::all(Val::Px(2.)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            margin: UiRect::vertical(Val::Px(10.)),
            border_radius: BorderRadius::all(Val::Percent(50.)),
            ..Default::default()
        },
        BackgroundColor(BUTTON_COLOR),
        BorderColor::all(Color::Srgba(BLACK)),
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
    let restart_text_bundle = (Text::new("Restart"), TextColor(Color::Srgba(WHITE)));
    let menu_button_bundle = (
        Node {
            display: Display::Flex,
            border: UiRect::all(Val::Px(2.)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            margin: UiRect::vertical(Val::Px(10.)),
            border_radius: BorderRadius::all(Val::Percent(50.)),
            ..Default::default()
        },
        BackgroundColor(BUTTON_COLOR),
        BorderColor::all(Color::Srgba(BLACK)),
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
    let menu_text_bundle = (Text::new("Main menu"), TextColor(Color::Srgba(WHITE)));
    commands.spawn(screen_node_bundle).with_children(|parent| {
        parent.spawn(game_over_text_node).with_children(|parent| {
            parent.spawn(game_over_text_bundle);
            parent.spawn(final_score_text_bundle);
        });
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
    if let Ok(entity) = pause_menu_query.single() {
        commands.entity(entity).despawn();
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
    if let Ok((interaction, mut background_color)) = button_query.single_mut() {
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
    if let Ok((interaction, mut background_color)) = button_query.single_mut() {
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
