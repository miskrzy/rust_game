use crate::game::player::components::Score;

use super::super::player::components::{Health, Player};
use super::{
    components::{GreenHealthBar, HUDNode, RedHealthBar, ScoreText},
    constants::HEALTH_BAR_LENGTH,
};
use bevy::hierarchy::ChildBuild;
use bevy::prelude::{DespawnRecursiveExt, Entity};
use bevy::text::TextColor;
use bevy::ui::{AlignItems, FlexDirection, JustifyContent, UiRect};
use bevy::{
    color::{
        palettes::{
            css::WHITE,
            tailwind::{GREEN_500, RED_500},
        },
        Color,
    },
    prelude::{BuildChildren, Commands, Node, Query, Text, With, Without},
    ui::{BackgroundColor, Display, PositionType, Val},
};

pub fn spawn(mut commands: Commands) {
    let top_node_bundle = (
        Node {
            display: Display::Flex,
            position_type: PositionType::Absolute,
            width: Val::Percent(100.),
            height: Val::Percent(10.),
            top: Val::Percent(0.),
            ..Default::default()
        },
        HUDNode,
    );
    let health_bar_green_bundle = (
        Node {
            display: Display::Flex,
            width: Val::Px(HEALTH_BAR_LENGTH),
            height: Val::Px(20.),
            top: Val::Px(10.),
            left: Val::Px(10.),
            ..Default::default()
        },
        BackgroundColor(Color::Srgba(GREEN_500)),
        GreenHealthBar,
    );
    let health_bar_red_bundle = (
        Node {
            display: Display::Flex,
            width: Val::Px(0.),
            height: Val::Px(20.),
            top: Val::Px(10.),
            left: Val::Px(10.),
            ..Default::default()
        },
        BackgroundColor(Color::Srgba(RED_500)),
        RedHealthBar,
    );
    let score_node = Node {
        display: Display::Flex,
        position_type: PositionType::Absolute,
        flex_direction: FlexDirection::Row,
        top: Val::Px(10.),
        right: Val::Px(10.),
        ..Default::default()
    };
    let score_text_node = Node {
        display: Display::Flex,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        margin: UiRect::all(Val::Px(10.)),
        ..Default::default()
    };
    let score_text_bundle = (Text::new("Score: "), TextColor(Color::Srgba(WHITE)));
    let points_text_node = Node {
        display: Display::Flex,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        margin: UiRect::all(Val::Px(10.)),
        ..Default::default()
    };
    let points_text_bundle = (Text::new("0"), TextColor(Color::Srgba(WHITE)), ScoreText);

    commands.spawn(top_node_bundle).with_children(|parent| {
        parent.spawn(health_bar_green_bundle);
        parent.spawn(health_bar_red_bundle);
        parent.spawn(score_node).with_children(|parent| {
            parent.spawn(score_text_node).with_child(score_text_bundle);
            parent
                .spawn(points_text_node)
                .with_child(points_text_bundle);
        });
    });
}

pub fn update_health_bar(
    player_query: Query<&Health, With<Player>>,
    mut green_health_bar_query: Query<&mut Node, (With<GreenHealthBar>, Without<RedHealthBar>)>,
    mut red_health_bar_query: Query<&mut Node, (With<RedHealthBar>, Without<GreenHealthBar>)>,
) {
    if let Ok(player_health) = player_query.get_single() {
        let current = player_health.current();
        let max = player_health.max();
        let fraction_green = current / max;
        let fraction_red = (max - current) / max;
        if let (Ok(mut green_bar), Ok(mut red_bar)) = (
            green_health_bar_query.get_single_mut(),
            red_health_bar_query.get_single_mut(),
        ) {
            green_bar.width = Val::Px(HEALTH_BAR_LENGTH * fraction_green);
            red_bar.width = Val::Px(HEALTH_BAR_LENGTH * fraction_red);
        }
    }
}

pub fn update_score(
    mut score_node: Query<&mut Text, With<ScoreText>>,
    player_query: Query<&Score, With<Player>>,
) {
    if let (Ok(mut text), Ok(score)) = (score_node.get_single_mut(), player_query.get_single()) {
        text.0 = score.score.to_string();
    }
}

pub fn despawn(hud_query: Query<Entity, With<HUDNode>>, mut commands: Commands) {
    if let Ok(entity) = hud_query.get_single() {
        commands.entity(entity).despawn_recursive();
    }
}
