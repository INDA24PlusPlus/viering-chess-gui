use bevy::prelude::*;

use super::resources::SoundEffects;

pub(crate) fn setup_resources(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(SoundEffects {
        select: asset_server.load("select.ogg"),
        capture: asset_server.load("capture.ogg"),
        illegal_move: asset_server.load("illegal.ogg"),
        valid_move: asset_server.load("move.ogg"),
        click: asset_server.load("click.ogg"),
        splash: asset_server.load("splash_sfx.ogg"),
    })
}
