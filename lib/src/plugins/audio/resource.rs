use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

#[derive(Resource)]
pub struct BackgroundMusic(pub Handle<AudioInstance>);
