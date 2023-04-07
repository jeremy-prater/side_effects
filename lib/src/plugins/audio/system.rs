use super::resource::*;
use super::state::*;
use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use std::time::Duration;

pub fn audio_state_changed(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    state: Res<State<AudioState>>,
    audio: Res<Audio>,
    mut audio_tracks: Option<ResMut<Assets<AudioInstance>>>,
    background_music: Option<Res<BackgroundMusic>>,
) {
    if !state.is_changed() {
        return;
    }
    info!("AudioState :: Audio state change to {:?}!", state);

    if let Some(mut audio_track) = audio_tracks {
        if let Some(mut background_music) = background_music {
            if let Some(instance) = audio_track.get_mut(&background_music.0) {
                instance.stop(AudioTween::linear(Duration::from_secs(3)));
            }
        }
    }

    match state.0 {
        AudioState::MenuIntro => {
            let background_music = audio
                .play(asset_server.load("audio/menu_intro.ogg"))
                .fade_in(AudioTween::linear(Duration::from_secs(3)))
                .handle();

            commands.insert_resource(BackgroundMusic(background_music));
        }
        AudioState::MenuLoop => {
            let background_music = audio
                .play(asset_server.load("audio/menu_loop.ogg"))
                .looped()
                .handle();

            commands.insert_resource(BackgroundMusic(background_music));
        }
        AudioState::AmbientIntro => {
            let background_music = audio
                .play(asset_server.load("audio/ambient_intro.ogg"))
                .fade_in(AudioTween::linear(Duration::from_secs(3)))
                .handle();

            commands.insert_resource(BackgroundMusic(background_music));
        }
        AudioState::AmbientLoop => {
            let background_music = audio
                .play(asset_server.load("audio/ambient_loop.ogg"))
                .looped()
                .handle();

            commands.insert_resource(BackgroundMusic(background_music));
        }
        AudioState::GoodIntro => {
            let background_music = audio
                .play(asset_server.load("audio/good_intro.ogg"))
                .fade_in(AudioTween::linear(Duration::from_secs(3)))
                .handle();

            commands.insert_resource(BackgroundMusic(background_music));
        }
        AudioState::GoodLoop => {
            let background_music = audio
                .play(asset_server.load("audio/good_loop.ogg"))
                .looped()
                .handle();

            commands.insert_resource(BackgroundMusic(background_music));
        }
        AudioState::BadIntro => {
            let background_music = audio
                .play(asset_server.load("audio/bad_intro.ogg"))
                .fade_in(AudioTween::linear(Duration::from_secs(3)))
                .handle();

            commands.insert_resource(BackgroundMusic(background_music));
        }
        AudioState::BadLoop => {
            let background_music = audio
                .play(asset_server.load("audio/bad_loop.ogg"))
                .looped()
                .handle();

            commands.insert_resource(BackgroundMusic(background_music));
        }
    }
}

pub fn audio_event_handler(
    mut commands: Commands,
    state: Res<State<AudioState>>,
    mut audio_tracks: ResMut<Assets<AudioInstance>>,
    background_music: Option<Res<BackgroundMusic>>,
) {
    if background_music.is_none() {
        return;
    }

    if let Some(instance) = audio_tracks.get_mut(&background_music.unwrap().0) {
        // info!("audio state: {:?}", instance.state());
        if instance.state() == PlaybackState::Stopped {
            commands.insert_resource(NextState(Some(match state.0 {
                AudioState::MenuIntro => AudioState::MenuLoop,
                AudioState::MenuLoop => AudioState::MenuLoop,
                AudioState::AmbientIntro => AudioState::AmbientLoop,
                AudioState::AmbientLoop => AudioState::AmbientLoop,
                AudioState::GoodIntro => AudioState::GoodLoop,
                AudioState::GoodLoop => AudioState::GoodLoop,
                AudioState::BadIntro => AudioState::BadLoop,
                AudioState::BadLoop => AudioState::BadLoop,
            })));
        }
    }
}
