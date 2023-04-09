use crate::plugins::camera::component::MainCamera;
use bevy::prelude::*;
use kayak_ui::prelude::{widgets::*, *};

pub fn setup_game_ui(
    mut commands: Commands,
    mut font_mapping: ResMut<FontMapping>,
    asset_server: Res<AssetServer>,
    camera: Query<Entity, With<MainCamera>>,
) {
    font_mapping.set_default(asset_server.load("roboto.kayak_font"));
    // font_mapping.force_subpixel(&asset_server.load("roboto.kayak_font"));
    let parent_id = None;
    rsx! {
        <KayakAppBundle>
            <TextWidgetBundle
                text={TextProps {
                    content: "Hello World".into(),
                    size: 20.0,
                    ..Default::default()
                }}
            />
        </KayakAppBundle>
    };

    commands.spawn((widget_context, EventDispatcher::default()));
}
