use bevy::prelude::*;

use crate::game::networking::Connection;

#[derive(Resource)]
pub struct SoundEffects {
    pub select: Handle<AudioSource>,
    pub capture: Handle<AudioSource>,
    pub valid_move: Handle<AudioSource>,
    pub illegal_move: Handle<AudioSource>,
    pub click: Handle<AudioSource>,
    pub splash: Handle<AudioSource>,
}

#[derive(PartialEq, Copy, Clone)]
pub enum NetworkRole {
    Server,
    Client,
}

#[derive(Resource)]
pub struct NetworkHandler {
    pub connection: Option<Connection>,
    pub role: NetworkRole,
    pub address_to_join: Option<String>,
}
