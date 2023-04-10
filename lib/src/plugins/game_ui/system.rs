use std::{collections::VecDeque, time::Duration};

use super::component::*;
use crate::{
    components::{
        animation::AnimationMarker,
        movement::{CharacterSpeed, Momentum, MovingCharacter},
        tanuki::Tanuki,
    },
    plugins::{
        ai::{FollowAction, FollowScorer, MoveToAction, MoveToScorer},
        mushroom_generator::{component::*, resource::MushroomDatabase},
        player::component::*,
        selection::*,
        DefaultFollowerJob, TanukiBundle,
    },
    systems::movement::IsInMotion,
};
use bevy::{prelude::*, utils::hashbrown::HashSet};
use bevy_rapier3d::prelude::*;
use big_brain::{prelude::Highest, thinker::Thinker};

const BUTTON_PADDING: UiRect = UiRect::all(Val::Percent(0.0));
const BUTTON_MARGIN: UiRect = UiRect::all(Val::Percent(2.0));
const BUTTON_BORDER: UiRect = UiRect::all(Val::Percent(0.0));

const BUTTON_DISABLED: BackgroundColor = BackgroundColor(Color::Rgba {
    red: 0.3,
    green: 0.3,
    blue: 0.3,
    alpha: 1.0,
});

const BUTTON_ENABLED: BackgroundColor = BackgroundColor(Color::FUCHSIA);

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
                    alpha: 0.0,
                }),
                ..default()
            });
            parent
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Percent(20.0)),
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::FlexStart,
                        ..default()
                    },
                    background_color: BackgroundColor(Color::Rgba {
                        red: 0.3,
                        green: 0.3,
                        blue: 0.3,
                        alpha: 0.5,
                    }),
                    ..default()
                })
                // Health Panel
                .with_children(|parent| {
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(30.0), Val::Percent(100.0)),
                                flex_direction: FlexDirection::Column,
                                justify_content: JustifyContent::FlexStart,
                                ..default()
                            },
                            background_color: BackgroundColor(Color::NONE),
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn((
                                TextBundle::from_section(
                                    "Health :",
                                    TextStyle {
                                        font: asset_server.load("fonts/arcade.ttf"),
                                        font_size: 30.0,
                                        color: Color::WHITE,
                                    },
                                )
                                .with_style(Style {
                                    padding: UiRect::left(Val::Percent(2.0)),
                                    margin: UiRect::all(Val::Percent(3.0)),
                                    ..default()
                                }),
                                HealthText,
                            ));
                        });
                })
                // Side effects panel
                .with_children(|parent| {
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(30.0), Val::Percent(100.0)),
                                flex_direction: FlexDirection::Column,
                                justify_content: JustifyContent::FlexStart,
                                ..default()
                            },
                            background_color: BackgroundColor(Color::NONE),
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn((
                                TextBundle::from_section(
                                    "Side Effects",
                                    TextStyle {
                                        font: asset_server.load("fonts/arcade.ttf"),
                                        font_size: 30.0,
                                        color: Color::WHITE,
                                    },
                                )
                                .with_style(Style {
                                    padding: UiRect::left(Val::Percent(2.0)),
                                    margin: UiRect::all(Val::Percent(3.0)),
                                    ..default()
                                }),
                                EffectsText,
                            ));
                        });
                })
                .with_children(|parent| {
                    parent
                        .spawn(ButtonBundle {
                            style: Style {
                                flex_direction: FlexDirection::Column,
                                justify_content: JustifyContent::Center,
                                size: Size::width(Val::Percent(8.0)),
                                padding: BUTTON_PADDING,
                                margin: BUTTON_MARGIN,
                                border: BUTTON_BORDER,
                                ..default()
                            },
                            background_color: BUTTON_DISABLED,
                            image: asset_server.load("icons/attack.png").into(),
                            ..default()
                        })
                        .insert(AttackButton::default());
                })
                .with_children(|parent| {
                    parent
                        .spawn(ButtonBundle {
                            style: Style {
                                flex_direction: FlexDirection::Column,
                                justify_content: JustifyContent::Center,
                                size: Size::width(Val::Percent(8.0)),
                                padding: BUTTON_PADDING,
                                margin: BUTTON_MARGIN,
                                border: BUTTON_BORDER,
                                ..default()
                            },
                            background_color: BUTTON_DISABLED,
                            image: asset_server.load("icons/inspect.png").into(),
                            ..default()
                        })
                        .insert(InspectButton::default());
                })
                .with_children(|parent| {
                    parent
                        .spawn(ButtonBundle {
                            style: Style {
                                flex_direction: FlexDirection::Column,
                                justify_content: JustifyContent::Center,
                                size: Size::width(Val::Percent(8.0)),
                                padding: BUTTON_PADDING,
                                margin: BUTTON_MARGIN,
                                border: BUTTON_BORDER,
                                ..default()
                            },
                            background_color: BUTTON_DISABLED,
                            image: asset_server.load("icons/move.png").into(),
                            ..default()
                        })
                        .insert(MoveButton::default());
                })
                .with_children(|parent| {
                    parent
                        .spawn(ButtonBundle {
                            style: Style {
                                flex_direction: FlexDirection::Column,
                                justify_content: JustifyContent::Center,
                                size: Size::width(Val::Percent(8.0)),
                                padding: BUTTON_PADDING,
                                margin: BUTTON_MARGIN,
                                border: BUTTON_BORDER,
                                ..default()
                            },
                            background_color: BUTTON_DISABLED,
                            image: asset_server.load("icons/pick.png").into(),
                            ..default()
                        })
                        .insert(PickButton::default());
                });
        })
        .insert(GameUI);
}

pub fn game_ui_system(
    active_selection: Query<&Transform, (With<Selected>, With<Mushroom>)>,
    player: Query<(&Transform, &Player)>,
    mut text_set: ParamSet<(
        Query<&mut Text, With<EffectsText>>,
        Query<&mut Text, With<HealthText>>,
    )>,
    mut button_set: ParamSet<(
        Query<(&mut BackgroundColor, &mut AttackButton), With<Button>>,
        Query<(&mut BackgroundColor, &mut MoveButton), With<Button>>,
        Query<(&mut BackgroundColor, &mut InspectButton), With<Button>>,
        Query<(&mut BackgroundColor, &mut PickButton), With<Button>>,
    )>,
    tanukis: Query<&Tanuki>,
) {
    let attack_enabled = false;
    let move_enabled = false;
    let mut inspect_enabled = false;
    let mut pick_enabled = false;

    if let Ok(player_pos) = player.get_single() {
        if let Ok(mut health_text) = text_set.p0().get_single_mut() {
            health_text.sections[0].value = format!(
                "Health : {}\nTanuki : {}",
                player_pos.1.hp as u32,
                tanukis.iter().len() + 1
            );
        }
        if let Ok(mut effect_text) = text_set.p1().get_single_mut() {
            let mut effect_string: String = "Side Effects".to_string();
            for effect in player_pos.1.active_effects.iter() {
                effect_string +=
                    format!("\n{:?} {}", effect.effect, effect.time_left.as_secs()).as_str();
            }
            effect_text.sections[0].value = effect_string;
        }

        for transform in active_selection.iter() {
            let distance = transform.translation - player_pos.0.translation;
            if distance.length() < 3.0 {
                pick_enabled = true;
                inspect_enabled = true;
            }
        }
    }

    if let Ok(mut button) = button_set.p0().get_single_mut() {
        button.1 .0 = attack_enabled;
        *button.0 = match attack_enabled {
            true => BUTTON_ENABLED,
            false => BUTTON_DISABLED,
        };
    }
    if let Ok(mut button) = button_set.p1().get_single_mut() {
        button.1 .0 = move_enabled;
        *button.0 = match move_enabled {
            true => BUTTON_ENABLED,
            false => BUTTON_DISABLED,
        };
    }
    if let Ok(mut button) = button_set.p2().get_single_mut() {
        button.1 .0 = inspect_enabled;
        *button.0 = match inspect_enabled {
            true => BUTTON_ENABLED,
            false => BUTTON_DISABLED,
        };
    }
    if let Ok(mut button) = button_set.p3().get_single_mut() {
        button.1 .0 = pick_enabled;
        *button.0 = match pick_enabled {
            true => BUTTON_ENABLED,
            false => BUTTON_DISABLED,
        };
    }
}

pub fn ui_button_handler(
    mut commands: Commands,
    mushroom_db: ResMut<MushroomDatabase>,
    mut player: Query<(&mut Player, &Transform)>,
    mut attack_button: Query<(&Interaction, &AttackButton), (Changed<Interaction>, With<Button>)>,
    mut move_button: Query<(&Interaction, &MoveButton), (Changed<Interaction>, With<Button>)>,
    mut inspect_button: Query<(&Interaction, &InspectButton), (Changed<Interaction>, With<Button>)>,
    mut pick_button: Query<(&Interaction, &PickButton), (Changed<Interaction>, With<Button>)>,
    mut active_mushroom: Query<(&Mushroom, Entity), With<Selected>>,
    active_selection: ResMut<ActiveSelection>,
    asset_server: Res<AssetServer>,
    default_job: Res<DefaultFollowerJob>,
) {
    if let Ok((interaction, enabled)) = attack_button.get_single_mut() {
        if enabled.0 && *interaction == Interaction::Clicked {
            info!("Attack clicked!");
        }
    }
    if let Ok((interaction, enabled)) = move_button.get_single_mut() {
        if enabled.0 && *interaction == Interaction::Clicked {
            info!("Move clicked!");
        }
    }
    if let Ok((interaction, enabled)) = pick_button.get_single_mut() {
        if enabled.0 && *interaction == Interaction::Clicked {
            if let Ok((mushroom, entity)) = active_mushroom.get_single_mut() {
                if let Ok((mut player, player_pos)) = player.get_single_mut() {
                    let (health, effect) =
                        mushroom.pick(entity, &mut commands, mushroom_db, active_selection);
                    info!("Eating mushroom {} {:?}", health, effect);

                    let old_hp = player.hp;

                    match effect {
                        MushroomEffect::NoSideEffect => {}
                        // MushroomEffect::Death => {
                        //     // You died
                        //     player.hp = 0.0;
                        // }
                        MushroomEffect::FullHealth => {
                            // Full power
                            player.hp = 100.0;
                        }
                        MushroomEffect::Sleep
                        | MushroomEffect::SpeedUp
                        | MushroomEffect::SlowDown => {
                            player.active_effects.push(ActiveMushroomEffect {
                                effect,
                                time_left: Duration::from_secs(30),
                            });
                        }

                        MushroomEffect::Minus5Health => {
                            player.hp -= 5.0;
                        }
                        MushroomEffect::Plus5Health => {
                            player.hp += 5.0;
                        }

                        MushroomEffect::SpawnEnemy => {
                            info!("Spawn enemy");
                        }

                        MushroomEffect::SpawnBuddy => {
                            let tanuki = commands
                                .spawn(TanukiBundle {
                                    scene: SceneBundle {
                                        scene: asset_server.load("models/Tanuki.glb#Scene0"),
                                        transform: Transform::from_xyz(
                                            player_pos.translation.x - 3.0,
                                            1.0,
                                            player_pos.translation.z - 3.0,
                                        )
                                        .with_scale(Vec3::new(0.5, 0.5, 0.5)),
                                        ..Default::default()
                                    },
                                    animation_marker: AnimationMarker::new("tanuki", "idle"),
                                    rigid_body: RigidBody::Dynamic,
                                    velocity: Velocity::default(),
                                    locked_axes: LockedAxes::ROTATION_LOCKED,
                                    collider: Collider::capsule_y(0.1, 0.2),
                                    direction: crate::components::movement::Direction::default(),
                                    momentum: Momentum::default(),
                                    damping: Damping {
                                        linear_damping: 0.2,
                                        angular_damping: 0.0,
                                    },
                                    friction: Friction {
                                        coefficient: 8.0,
                                        combine_rule: CoefficientCombineRule::Average,
                                    },
                                    gravity: GravityScale(1.0),
                                    tanuki: Tanuki {
                                        age: 1,
                                        max_hp: 1,
                                        hp: 1,
                                        current_effects: Vec::new(),
                                        blocked_effects: HashSet::new(),
                                        next_actions: VecDeque::new(),
                                    },
                                    selectable: Selectable,
                                    thinker: Thinker::build()
                                        .label("tanuki_ai")
                                        .picker(Highest)
                                        .when(
                                            FollowScorer,
                                            FollowAction {
                                                end_pos: None,
                                                path_cache: Vec::new().into_iter().peekable(),
                                            },
                                        )
                                        .when(
                                            MoveToScorer,
                                            MoveToAction {
                                                end_pos: None,
                                                path_cache: Vec::new().into_iter().peekable(),
                                            },
                                        ),
                                    motion_tracker: IsInMotion(false),
                                    moving_char_tag: MovingCharacter,
                                    character_speed: CharacterSpeed::default(),
                                })
                                .id();
                            commands.entity(default_job.0).add_child(tanuki);
                        }
                    }

                    player.hp += health;

                    if old_hp < 100.0 && player.hp > 100.00 {
                        commands.insert_resource(NextState(Some(
                            crate::plugins::audio::state::AudioState::GoodIntro,
                        )));
                    }
                }
            }
        }
    }
    if let Ok((interaction, enabled)) = inspect_button.get_single_mut() {
        if enabled.0 && *interaction == Interaction::Clicked {
            info!("Inspect clicked!");
        }
    }
}
