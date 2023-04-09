use super::resource::LoadingScreen;
use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use iyes_progress::prelude::*;
use log::info;

pub fn load_game_assets(asset_server: Res<AssetServer>, mut loading: ResMut<AssetsLoading>) {
    info!("Loading game assets");

    // Fonts
    let arcade_font: Handle<Font> = asset_server.load("fonts/arcade.ttf");
    loading.add(&arcade_font);
    let freedom_font: Handle<Font> = asset_server.load("fonts/freedom.ttf");
    loading.add(&freedom_font);

    // Tanuki
    let tanuki_model: Handle<Scene> = asset_server.load("models/Tanuki.glb#Scene0");
    loading.add(&tanuki_model);

    // Mushrooms
    let mushrooom2_model: Handle<Scene> = asset_server.load("models/mushroom2.glb#Scene0");
    loading.add(&mushrooom2_model);

    // Trees
    let tree_model: Handle<Scene> = asset_server.load("models/tree_1.glb#Scene0");
    loading.add(&tree_model);
    
    // Music
    let ambient_intro: Handle<AudioSource> = asset_server.load("audio/ambient_intro.ogg");
    loading.add(&ambient_intro);
    let ambient_loop: Handle<AudioSource> = asset_server.load("audio/ambient_loop.ogg");
    loading.add(&ambient_loop);
    let bad_trip_intro: Handle<AudioSource> = asset_server.load("audio/bad_trip_intro.ogg");
    loading.add(&bad_trip_intro);
    let bad_trip_loop: Handle<AudioSource> = asset_server.load("audio/bad_trip_loop.ogg");
    loading.add(&bad_trip_loop);
    let good_trip_intro: Handle<AudioSource> = asset_server.load("audio/good_trip_intro.ogg");
    loading.add(&good_trip_intro);
    let good_trip_loop: Handle<AudioSource> = asset_server.load("audio/good_trip_loop.ogg");
    loading.add(&good_trip_loop);
    let menu_intro: Handle<AudioSource> = asset_server.load("audio/menu_intro.ogg");
    loading.add(&menu_intro);
    let menu_loop: Handle<AudioSource> = asset_server.load("audio/menu_loop.ogg");
    loading.add(&menu_loop);

    // Icons
    let attack_icon: Handle<Image> = asset_server.load("icons/attack.png");
    loading.add(attack_icon);
    let inspect_icon: Handle<Image> = asset_server.load("icons/inspect.png");
    loading.add(inspect_icon);
    let move_icon: Handle<Image> = asset_server.load("icons/move.png");
    loading.add(move_icon);
    let pick_icon: Handle<Image> = asset_server.load("icons/pick.png");
    loading.add(pick_icon);
    let wood_icon: Handle<Image> = asset_server.load("icons/wood.png");
    loading.add(wood_icon);
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
