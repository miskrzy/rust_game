use super::super::player::components::{Health, Player};
use super::{
    components::{GreenHealthBar, RedHealthBar, ScoreNode},
    constants::HEALTH_BAR_LENGTH,
    resources::Score,
};
use bevy::{
    color::{
        palettes::{
            css::WHITE,
            tailwind::{GREEN_500, RED_500},
        },
        Color,
    },
    prelude::{BuildChildren, Commands, NodeBundle, Query, Res, TextBundle, With, Without},
    text::{Text, TextSection, TextStyle},
    ui::{BackgroundColor, Display, PositionType, Style, Val},
};

pub fn spawn(mut commands: Commands) {
    let top_node = NodeBundle {
        style: Style {
            display: Display::Flex,
            position_type: PositionType::Absolute,
            width: Val::Percent(100.),
            height: Val::Percent(10.),
            top: Val::Percent(0.),
            ..Default::default()
        },
        ..Default::default()
    };
    let health_bar_green = NodeBundle {
        style: Style {
            display: Display::Flex,
            width: Val::Px(HEALTH_BAR_LENGTH),
            height: Val::Px(20.),
            top: Val::Px(10.),
            left: Val::Px(10.),
            ..Default::default()
        },
        background_color: BackgroundColor(Color::Srgba(GREEN_500)),
        ..Default::default()
    };
    let health_bar_red = NodeBundle {
        style: Style {
            display: Display::Flex,
            width: Val::Px(0.),
            height: Val::Px(20.),
            top: Val::Px(10.),
            left: Val::Px(10.),
            ..Default::default()
        },
        background_color: BackgroundColor(Color::Srgba(RED_500)),
        ..Default::default()
    };
    let score = TextBundle::from_sections([
        TextSection::new(
            "Score: ",
            TextStyle {
                color: Color::Srgba(WHITE),
                ..Default::default()
            },
        ),
        TextSection::new(
            "0",
            TextStyle {
                color: Color::Srgba(WHITE),
                ..Default::default()
            },
        ),
    ])
    .with_style(Style {
        display: Display::Flex,
        position_type: PositionType::Absolute,
        top: Val::Px(10.),
        right: Val::Px(10.),
        ..Default::default()
    });
    commands.spawn(top_node).with_children(|parent| {
        parent.spawn((health_bar_green, GreenHealthBar));
        parent.spawn((health_bar_red, RedHealthBar));
        parent.spawn((score, ScoreNode));
    });
}

pub fn update_health_bar(
    player_query: Query<&Health, With<Player>>,
    mut green_health_bar_query: Query<&mut Style, (With<GreenHealthBar>, Without<RedHealthBar>)>,
    mut red_health_bar_query: Query<&mut Style, (With<RedHealthBar>, Without<GreenHealthBar>)>,
) {
    if let Ok(player_health) = player_query.get_single() {
        let current = player_health.current();
        let max = player_health.max();
        let fraction_green = current / max;
        let fraction_red = (max - current) / max;
        let mut green_bar = green_health_bar_query.get_single_mut().unwrap();
        let mut red_bar = red_health_bar_query.get_single_mut().unwrap();
        green_bar.width = Val::Px(HEALTH_BAR_LENGTH * fraction_green);
        red_bar.width = Val::Px(HEALTH_BAR_LENGTH * fraction_red);
    }
}

pub fn update_score(mut score_node: Query<&mut Text, With<ScoreNode>>, score: Res<Score>) {
    let mut text = score_node.get_single_mut().unwrap();
    text.sections[1].value = score.score.to_string();
}
