use bevy::prelude::*;

#[derive(Resource)]
pub struct SoundEffects {
    pub select: Handle<AudioSource>,
    pub capture: Handle<AudioSource>,
    pub valid_move: Handle<AudioSource>,
    pub illegal_move: Handle<AudioSource>,
    pub click: Handle<AudioSource>,
    pub splash: Handle<AudioSource>,
}
