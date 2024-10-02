use bevy::prelude::*;

use super::resources::{NetworkHandler, NetworkRole, SoundEffects};

pub(crate) fn setup_resources(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(SoundEffects {
        select: asset_server.load("audio/select.ogg"),
        capture: asset_server.load("audio/capture.ogg"),
        illegal_move: asset_server.load("audio/illegal.ogg"),
        valid_move: asset_server.load("audio/move.ogg"),
        click: asset_server.load("audio/click.ogg"),
        splash: asset_server.load("audio/splash_screen.ogg"),
    });

    commands.insert_resource(NetworkHandler {
        connection: None,
        role: NetworkRole::Client,
    });
}
