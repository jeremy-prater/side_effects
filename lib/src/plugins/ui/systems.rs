use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier3d::prelude::*;
use kayak_ui::{prelude::*, widgets::*};

pub fn setup_ui(cameras: Query<(Entity, &Camera)>, mut commands: Commands) {}

// /// Utility that generally handles everything to do with the radial menu used to order jobs. It
// /// opens a radial menu or keeps the radial menu open for the duration of a held click, and sends
// /// an event if the held click is released over one of the options.
// pub fn handle_radial_menu(
//     windows: Query<&Window, With<PrimaryWindow>>,
//     cameras: Query<(Entity, &Camera, &GlobalTransform)>,
//     radial_menu: Query<Entity, With<RadialMenu>>,

//     rapier_context: Res<RapierContext>,
//     asset_server: Res<AssetServer>,
//     mouse_buttons: Res<Input<MouseButton>>,
//     mut font_mapping: ResMut<FontMapping>,

//     mut commands: Commands,
// ) {
//     if mouse_buttons.just_pressed(MouseButton::Left) {
//         let window = windows.single();

//         if let Some(mut pos) = window.cursor_position() {
//             pos.y = window.resolution.height() - pos.y;

//             let camera = cameras.single().0;

//             font_mapping.set_default(asset_server.load("fonts/roboto.kayak_font"));

//             let mut widget_context = KayakRootContext::new(camera);
//             widget_context.add_plugin(KayakWidgetsContextPlugin);
//             let parent_id = None;

//             rsx! {
//                 <KayakAppBundle>
//                     <BackgroundBundle
//                            background={Background::default()}
//                            styles={KStyle {
//                                background_color: Color::DARK_GRAY.into(),
//                                border_radius: Corner::all(100.0).into(),
//                                width: StyleProp::Value(Units::Pixels(150.0)),
//                                height: StyleProp::Value(Units::Pixels(150.0)),
//                                left: StyleProp::Value(Units::Pixels(pos.x - 75.0)),
//                                top: StyleProp::Value(Units::Pixels(pos.y - 75.0)),
//                                ..Default::default()
//                            }}
//                     />
//                 </KayakAppBundle>
//             };

//             let world_pos =
//                 if let Some(world_pos) = cursor_world_position(windows, cameras, rapier_context) {
//                     world_pos
//                 } else {
//                     return;
//                 };

//             commands.spawn((
//                 widget_context,
//                 EventDispatcher::default(),
//                 RadialMenu { world_pos },
//             ));
//         }
//     }

//     if mouse_buttons.just_released(MouseButton::Left) {
//         despawn_with(commands, radial_menu)
//     }
// }
