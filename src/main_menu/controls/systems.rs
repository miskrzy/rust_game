use bevy::{
    color::{
        palettes::{
            css::{BLACK, GRAY, WHITE},
            tailwind::{GRAY_500, GRAY_700},
        },
        Color,
    },
    prelude::{
        BuildChildren, Button, Changed, ChildBuild, Commands, DespawnRecursiveExt, Entity,
        NextState, Node, Query, ResMut, Text, With,
    },
    text::TextColor,
    ui::{
        AlignItems, BackgroundColor, BorderColor, BorderRadius, Display, FlexDirection,
        Interaction, JustifyContent, Overflow, PositionType, UiRect, Val,
    },
};

use crate::main_menu::states::MainMenuState;

use super::{
    components::{ControlsScreen, ReturnButton},
    constants::{BUTTON_COLOR, BUTTON_HOVERED_COLOR, CONTROLS},
};

pub fn spawn(mut commands: Commands) {
    let screen_bundle = (
        Node {
            display: Display::Flex,
            position_type: PositionType::Absolute,
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            flex_direction: FlexDirection::Column,
            ..Default::default()
        },
        BackgroundColor(Color::Srgba(GRAY)),
        ControlsScreen,
    );

    let controls_bundle = (
        Node {
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
        BackgroundColor(Color::Srgba(GRAY)),
    );

    let buttons_bundle = (
        Node {
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
        BackgroundColor(Color::Srgba(GRAY)),
    );

    let return_button_bundle = (
        Node {
            display: Display::Flex,
            border: UiRect::all(Val::Px(2.)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            margin: UiRect::vertical(Val::Px(2.)),
            ..Default::default()
        },
        BackgroundColor(BUTTON_COLOR),
        BorderColor(Color::Srgba(BLACK)),
        BorderRadius::all(Val::Percent(50.)),
        Button,
        ReturnButton,
    );

    let return_text_node = Node {
        display: Display::Flex,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        margin: UiRect::all(Val::Px(10.)),
        ..Default::default()
    };
    let return_text_bundle = (
        Text::new("Return"),
        TextColor(Color::Srgba(WHITE)),
        
    );

    let mut row_nodes = Vec::new();
    for (description, key) in CONTROLS {
        row_nodes.push((
            (
                Node {
                    display: Display::Flex,
                    width: Val::Percent(90.),
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    margin: UiRect::vertical(Val::Px(5.)),
                    ..Default::default()
                },
                BorderRadius::all(Val::Percent(50.)),
                BackgroundColor(Color::Srgba(GRAY_700)),
            ),
            (
                Node {
                    display: Display::Flex,
                    width: Val::Percent(60.),
                    left: Val::Percent(1.),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    margin: UiRect::vertical(Val::Px(5.)),
                    ..Default::default()
                },
                BorderRadius::all(Val::Percent(50.)),
                BackgroundColor(Color::Srgba(GRAY_500)),
            ),
            Node {
                display: Display::Flex,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: UiRect::all(Val::Px(10.)),
                ..Default::default()
            },
            (
                Text::new(description),
                TextColor(Color::Srgba(WHITE)),
                // margin: UiRect::all(Val::Px(5.)),
            ),
            (
                Node {
                    display: Display::Flex,
                    position_type: PositionType::Absolute,
                    right: Val::Percent(1.),
                    width: Val::Percent(37.),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    margin: UiRect::vertical(Val::Px(5.)),
                    ..Default::default()
                },
                BorderRadius::all(Val::Percent(50.)),
                BackgroundColor(Color::Srgba(GRAY_500)),
            ),
            Node {
                display: Display::Flex,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: UiRect::all(Val::Px(10.)),
                ..Default::default()
            },
            (
                Text::new(key),
                TextColor(Color::Srgba(WHITE)),
                // margin: UiRect::all(Val::Px(5.)),
            ),
        ));
    }
    commands.spawn(screen_bundle).with_children(|parent| {
        parent.spawn(controls_bundle).with_children(|parent| {
            for (
                row_node,
                description_node,
                description_text_node,
                description_text_bundle,
                key_node,
                key_text_node,
                key_text_bundle,
            ) in row_nodes
            {
                parent.spawn(row_node).with_children(|parent| {
                    parent.spawn(description_node).with_children(|parent| {
                        parent
                            .spawn(description_text_node)
                            .with_child(description_text_bundle);
                    });
                    parent.spawn(key_node).with_children(|parent| {
                        parent.spawn(key_text_node).with_child(key_text_bundle);
                    });
                });
            }
        });
        parent.spawn(buttons_bundle).with_children(|parent| {
            parent.spawn(return_button_bundle).with_children(|parent| {
                parent
                    .spawn(return_text_node)
                    .with_child(return_text_bundle);
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
