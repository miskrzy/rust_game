use bevy::{
    color::{
        palettes::css::{BLACK, GRAY, WHITE},
        Alpha, Color,
    },
    input::ButtonInput,
    prelude::{
        BuildChildren, ButtonBundle, Changed, Commands, DespawnRecursiveExt, Entity, KeyCode,
        NextState, NodeBundle, Query, Res, ResMut, State, TextBundle, With,
    },
    text::{Text, TextStyle},
    ui::{
        AlignItems, BackgroundColor, BorderColor, BorderRadius, Display, FlexDirection,
        Interaction, JustifyContent, PositionType, Style, UiRect, Val,
    },
};

use crate::{game::states::GameState, states::AppState};

use super::{
    components::{PauseMenu, QuitButton, ResumeButton},
    constants::{BUTTON_COLOR, BUTTON_HOVERED_COLOR},
};

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
        background_color: BackgroundColor(Color::Srgba(GRAY).with_alpha(0.5)),
        ..Default::default()
    };
    let resume_button_node = ButtonBundle {
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
    let resume_text = TextBundle {
        text: Text::from_section(
            "Resume",
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
            "Quit",
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
        .spawn((screen_node, PauseMenu))
        .with_children(|parent| {
            parent
                .spawn((resume_button_node, ResumeButton))
                .with_children(|parent| {
                    parent.spawn(resume_text);
                });
            parent
                .spawn((quit_button_node, QuitButton))
                .with_children(|parent| {
                    parent.spawn(quit_text);
                });
        });
}

pub fn despawn(mut commands: Commands, pause_menu_query: Query<Entity, With<PauseMenu>>) {
    if let Ok(entity) = pause_menu_query.get_single() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn change_game_state(
    mut next_game_state: ResMut<NextState<GameState>>,
    game_state: ResMut<State<GameState>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyP) {
        match *game_state.get() {
            GameState::Play => {
                next_game_state.set(GameState::Pause);
            }
            GameState::Pause => {
                next_game_state.set(GameState::Play);
            }
        }
    }
}

pub fn resume_button_interaction(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (With<ResumeButton>, Changed<Interaction>),
    >,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                next_game_state.set(GameState::Play);
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
    mut next_game_state: ResMut<NextState<GameState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                next_game_state.set(GameState::Play);
                next_app_state.set(AppState::MainMenu);
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
