use bevy::{
    color::{
        palettes::{
            css::{BLACK, GRAY, WHITE},
            tailwind::{GRAY_500, GRAY_700},
        },
        Color,
    },
    prelude::{
        BuildChildren, ButtonBundle, Changed, Commands, DespawnRecursiveExt, Entity, NextState,
        NodeBundle, Query, ResMut, TextBundle, With,
    },
    text::{Text, TextStyle},
    ui::{
        AlignItems, BackgroundColor, BorderColor, BorderRadius, Display, FlexDirection,
        Interaction, JustifyContent, Overflow, PositionType, Style, UiRect, Val,
    },
};

use crate::main_menu::states::MainMenuState;

use super::{
    components::{ControlsScreen, ReturnButton},
    constants::{BUTTON_COLOR, BUTTON_HOVERED_COLOR, CONTROLS},
};

pub fn spawn(mut commands: Commands) {
    let screen_node = NodeBundle {
        style: Style {
            display: Display::Flex,
            position_type: PositionType::Absolute,
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            flex_direction: FlexDirection::Column,
            ..Default::default()
        },
        background_color: BackgroundColor(Color::Srgba(GRAY)),
        ..Default::default()
    };

    let controls_node = NodeBundle {
        style: Style {
            display: Display::Flex,
            position_type: PositionType::Absolute,
            width: Val::Percent(100.),
            height: Val::Percent(80.),
            top: Val::Percent(0.),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            overflow: Overflow::clip_y(),
            ..Default::default()
        },
        background_color: BackgroundColor(Color::Srgba(GRAY)),
        ..Default::default()
    };

    let buttons_node = NodeBundle {
        style: Style {
            display: Display::Flex,
            position_type: PositionType::Absolute,
            width: Val::Percent(100.),
            height: Val::Percent(20.),
            bottom: Val::Percent(0.),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        background_color: BackgroundColor(Color::Srgba(GRAY)),
        ..Default::default()
    };

    let return_button = ButtonBundle {
        style: Style {
            display: Display::Flex,
            border: UiRect::all(Val::Px(2.)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            margin: UiRect::vertical(Val::Px(2.)),
            ..Default::default()
        },
        background_color: BackgroundColor(BUTTON_COLOR),
        border_color: BorderColor(Color::Srgba(BLACK)),
        border_radius: BorderRadius::all(Val::Percent(50.)),
        ..Default::default()
    };

    let return_text = TextBundle {
        text: Text::from_section(
            "Return",
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

    let mut row_nodes: Vec<(NodeBundle, NodeBundle, TextBundle, NodeBundle, TextBundle)> =
        Vec::new();
    for (description, key) in CONTROLS {
        row_nodes.push((
            NodeBundle {
                style: Style {
                    display: Display::Flex,
                    width: Val::Percent(90.),
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    margin: UiRect::vertical(Val::Px(5.)),
                    ..Default::default()
                },
                border_radius: BorderRadius::all(Val::Percent(50.)),
                background_color: BackgroundColor(Color::Srgba(GRAY_700)),
                ..Default::default()
            },
            NodeBundle {
                style: Style {
                    display: Display::Flex,
                    width: Val::Percent(60.),
                    left: Val::Percent(1.),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    margin: UiRect::vertical(Val::Px(5.)),
                    ..Default::default()
                },
                border_radius: BorderRadius::all(Val::Percent(50.)),
                background_color: BackgroundColor(Color::Srgba(GRAY_500)),
                ..Default::default()
            },
            TextBundle {
                text: Text::from_section(
                    description,
                    TextStyle {
                        color: Color::Srgba(WHITE),
                        ..Default::default()
                    },
                ),
                style: Style {
                    margin: UiRect::all(Val::Px(5.)),
                    ..Default::default()
                },
                ..Default::default()
            },
            NodeBundle {
                style: Style {
                    display: Display::Flex,
                    position_type: PositionType::Absolute,
                    right: Val::Percent(1.),
                    width: Val::Percent(37.),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    margin: UiRect::vertical(Val::Px(5.)),
                    ..Default::default()
                },
                border_radius: BorderRadius::all(Val::Percent(50.)),
                background_color: BackgroundColor(Color::Srgba(GRAY_500)),
                ..Default::default()
            },
            TextBundle {
                text: Text::from_section(
                    key,
                    TextStyle {
                        color: Color::Srgba(WHITE),
                        ..Default::default()
                    },
                ),
                style: Style {
                    margin: UiRect::all(Val::Px(5.)),
                    ..Default::default()
                },
                ..Default::default()
            },
        ));
    }
    commands
        .spawn((screen_node, ControlsScreen))
        .with_children(|parent| {
            parent.spawn(controls_node).with_children(|parent| {
                for (row_node, description_node, description_text, key_node, key_text) in row_nodes
                {
                    parent.spawn(row_node).with_children(|parent| {
                        parent.spawn(description_node).with_children(|parent| {
                            parent.spawn(description_text);
                        });
                        parent.spawn(key_node).with_children(|parent| {
                            parent.spawn(key_text);
                        });
                    });
                }
            });
            parent.spawn(buttons_node).with_children(|parent| {
                parent
                    .spawn((return_button, ReturnButton))
                    .with_children(|parent| {
                        parent.spawn(return_text);
                    });
            });
        });
}

pub fn despawn(mut commands: Commands, controls_query: Query<Entity, With<ControlsScreen>>) {
    if let Ok(entity) = controls_query.get_single() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn return_button_interaction(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (With<ReturnButton>, Changed<Interaction>),
    >,
    mut next_main_menu_state: ResMut<NextState<MainMenuState>>,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                next_main_menu_state.set(MainMenuState::Home);
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
