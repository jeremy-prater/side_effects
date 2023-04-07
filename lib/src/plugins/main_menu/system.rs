use super::component::*;
use bevy::prelude::*;
use log::info;

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

pub fn setup_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    info!("setup menu!");
    commands
        .spawn(Camera3dBundle {
            transform: Transform::from_xyz(15.0, 0.0, 0.0)
                .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
            ..default()
        })
        .insert(Camera);

    let title = commands
        .spawn(TextBundle {
            text: Text::from_section(
                "The Tanukis' Trip",
                TextStyle {
                    font: asset_server.load("fonts/freedom.ttf"),
                    font_size: 80.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            ),
            style: Style {
                justify_content: JustifyContent::Center,
                padding: UiRect {
                    left: Val::Percent(5.),
                    right: Val::Percent(5.),
                    top: Val::Percent(5.),
                    bottom: Val::Percent(5.),
                },
                ..default()
            },
            ..default()
        })
        .insert(Title)
        .id();

    let exit_button = commands
        .spawn(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(250.0), Val::Px(65.0)),
                // center button
                margin: UiRect::all(Val::Auto),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: NORMAL_BUTTON.into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text::from_section(
                    "Start",
                    TextStyle {
                        font: asset_server.load("fonts/arcade.ttf"),
                        font_size: 40.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                ),
                ..default()
            });
        })
        .insert(ExitButton)
        .id();
    let menu = commands
        .spawn(NodeBundle {
            background_color: BackgroundColor(Color::rgb(0.5, 0.5, 0.5)),
            style: Style {
                size: Size::new(Val::Auto, Val::Auto),
                margin: UiRect::all(Val::Auto),
                align_self: AlignSelf::Center,
                flex_direction: FlexDirection::ColumnReverse,
                //align_items: AlignItems::Stretch,
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(MainMenu)
        .id();

    commands.entity(menu).push_children(&[exit_button, title]);
}

#[allow(clippy::type_complexity)]
pub fn main_menu_ui_system(
    mut commands: Commands,
    // _ev: EventWriter<AppExit>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    // mut update_name_query: Query<(&mut BackgroundColor, &Children), With<Button>>,
    mut text_query: Query<&mut Text>,
) {
    for (interaction, mut color, children) in interaction_query.iter_mut() {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                text.sections[0].value = "Start".to_string();
                *color = PRESSED_BUTTON.into();
                commands.insert_resource(NextState(Some(
                    crate::states::game_state::GameState::InGame,
                )));
                commands.insert_resource(NextState(Some(
                    crate::plugins::audio::state::AudioState::AmbientIntro,
                )));
            }
            Interaction::Hovered => {
                text.sections[0].value = "Start".to_string();
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                text.sections[0].value = "Start".to_string();
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}
