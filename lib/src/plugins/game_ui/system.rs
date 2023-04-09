use bevy::prelude::*;
use log::info;

const BUTTON_PADDING: UiRect = UiRect::all(Val::Percent(0.0));
const BUTTON_MARGIN: UiRect = UiRect::all(Val::Percent(2.0));
const BUTTON_BORDER: UiRect = UiRect::all(Val::Percent(0.0));

pub fn game_ui_startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // root node
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::width(Val::Percent(100.0)),
                justify_content: JustifyContent::SpaceBetween,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        })
        // Top panel
        .with_children(|parent| {
            parent.spawn(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Percent(15.0)),
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::Center,

                    ..default()
                },
                background_color: BackgroundColor(Color::Rgba {
                    red: 0.3,
                    green: 0.3,
                    blue: 0.3,
                    alpha: 0.7,
                }),
                ..default()
            });
            parent
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Percent(20.0)),
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::FlexEnd,
                        ..default()
                    },
                    background_color: BackgroundColor(Color::Rgba {
                        red: 0.0,
                        green: 0.0,
                        blue: 0.0,
                        alpha: 0.3,
                    }),
                    ..default()
                })
                .with_children(|parent| {
                    // Title
                    parent.spawn(ImageBundle {
                        style: Style {
                            flex_direction: FlexDirection::Column,
                            justify_content: JustifyContent::Center,
                            size: Size::width(Val::Percent(8.0)),
                            padding: BUTTON_PADDING,
                            margin: BUTTON_MARGIN,
                            border: BUTTON_BORDER,
                            ..default()
                        },
                        // background_color: asset_server.load("icons/wood.png").into(),
                        image: asset_server.load("icons/attack.png").into(),
                        ..default()
                    });
                })
                .with_children(|parent| {
                    // Title
                    parent.spawn(ImageBundle {
                        style: Style {
                            flex_direction: FlexDirection::Column,
                            justify_content: JustifyContent::Center,
                            size: Size::width(Val::Percent(8.0)),
                            padding: BUTTON_PADDING,
                            margin: BUTTON_MARGIN,
                            border: BUTTON_BORDER,
                            ..default()
                        },
                        image: asset_server.load("icons/inspect.png").into(),
                        ..default()
                    });
                })
                .with_children(|parent| {
                    // Title
                    parent.spawn(ImageBundle {
                        style: Style {
                            flex_direction: FlexDirection::Column,
                            justify_content: JustifyContent::Center,
                            size: Size::width(Val::Percent(8.0)),
                            padding: BUTTON_PADDING,
                            margin: BUTTON_MARGIN,
                            border: BUTTON_BORDER,
                            ..default()
                        },
                        image: asset_server.load("icons/move.png").into(),
                        ..default()
                    });
                })
                .with_children(|parent| {
                    // Title
                    parent.spawn(ImageBundle {
                        style: Style {
                            flex_direction: FlexDirection::Column,
                            justify_content: JustifyContent::Center,
                            size: Size::width(Val::Percent(8.0)),
                            padding: BUTTON_PADDING,
                            margin: BUTTON_MARGIN,
                            border: BUTTON_BORDER,
                            ..default()
                        },
                        image: asset_server.load("icons/pick.png").into(),
                        ..default()
                    });
                });
        });
}
