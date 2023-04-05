use super::resource::LoadingScreen;
use bevy::prelude::*;
use iyes_progress::prelude::*;
use log::info;

pub fn load_game_assets(asset_server: Res<AssetServer>, mut loading: ResMut<AssetsLoading>) {
    info!("Loading game assets");

    let font: Handle<Font> = asset_server.load("fonts/ARCADE.TTF");
    let tanuki_model: Handle<Scene> = asset_server.load("models/Tanuki.glb#Scene0");
    let mushrooom2_model: Handle<Scene> = asset_server.load("models/mushroom2.glb#Scene0");

    loading.add(&font);
    loading.add(&tanuki_model);
    loading.add(&mushrooom2_model);
}

pub fn setup_loading_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Camera
    let camera = commands.spawn(Camera2dBundle::default()).id();

    // root node
    let background = commands
        .spawn(ImageBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                ..default()
            },
            image: asset_server.load("loading.png").into(),
            ..default()
        })
        .id();

    commands.insert_resource(LoadingScreen { camera, background })
}

pub fn teardown_loading_ui(mut commands: Commands, loading: Res<LoadingScreen>) {
    info!("Asset loading complete. Tearing down loading screen");
    commands.entity(loading.background).despawn_recursive();
    commands.entity(loading.camera).despawn_recursive();
    commands.remove_resource::<LoadingScreen>();
}
