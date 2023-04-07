pub mod resource;
pub mod state;
pub mod system;

use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

use state::*;
use system::*;

pub struct GameAudioPlugin;

impl Plugin for GameAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(AudioPlugin)
            .add_state::<AudioState>()
            .add_system(audio_state_changed)
            .add_system(audio_event_handler);
    }
}
