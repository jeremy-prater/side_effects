use super::resource::LoadingScreen;
use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use iyes_progress::prelude::*;
use log::info;

pub fn load_game_assets(asset_server: Res<AssetServer>, mut loading: ResMut<AssetsLoading>) {
    info!("Loading game assets");

    let arcade_font: Handle<Font> = asset_server.load("fonts/arcade.ttf");
    let freedom_font: Handle<Font> = asset_server.load("fonts/freedom.ttf");
    let tanuki_model: Handle<Scene> = asset_server.load("models/Tanuki.glb#Scene0");
    let mushrooom2_model: Handle<Scene> = asset_server.load("models/mushroom2.glb#Scene0");
    let tree_model: Handle<Scene> = asset_server.load("models/tree_1.glb#Scene0");
    let ambient_intro: Handle<AudioSource> = asset_server.load("audio/ambient_intro.ogg");
    let ambient_loop: Handle<AudioSource> = asset_server.load("audio/ambient_loop.ogg");
    let bad_trip_intro: Handle<AudioSource> = asset_server.load("audio/bad_trip_intro.ogg");
    let bad_trip_loop: Handle<AudioSource> = asset_server.load("audio/bad_trip_loop.ogg");
    let good_trip_intro: Handle<AudioSource> = asset_server.load("audio/good_trip_intro.ogg");
    let good_trip_loop: Handle<AudioSource> = asset_server.load("audio/good_trip_loop.ogg");
    let menu_intro: Handle<AudioSource> = asset_server.load("audio/menu_intro.ogg");
    let menu_loop: Handle<AudioSource> = asset_server.load("audio/menu_loop.ogg");

    loading.add(&arcade_font);
    loading.add(&freedom_font);
    loading.add(&tanuki_model);
    loading.add(&mushrooom2_model);
    loading.add(&tree_model);
    loading.add(&ambient_intro);
    loading.add(&ambient_loop);
    loading.add(&bad_trip_intro);
    loading.add(&bad_trip_loop);
    loading.add(&good_trip_intro);
    loading.add(&good_trip_loop);
    loading.add(&menu_intro);
    loading.add(&menu_loop);
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
