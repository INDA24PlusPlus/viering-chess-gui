use std::{io::Write, time::Duration};

use bevy::prelude::*;
use bevy_mod_picking::PickableBundle;
use chess_networking::Start;

use crate::{
    game::{
        board_id_to_world_pos, networking::Connection, ChessSquare, ClientGameState, OnGameScreen,
        PieceModelData, SquareResourceData,
    },
    general::resources::{NetworkHandler, NetworkRole},
};

use super::board;

pub fn setup_game_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    piece_model_data: Res<PieceModelData>,
    square_resource_data: Res<SquareResourceData>,
    mut game_state: ResMut<ClientGameState>,
    mut network_handler: ResMut<NetworkHandler>,
) {
    // TODO TEMPORARY MOVE THIS IT IS NOT PART OF SCENE
    match network_handler.role {
        NetworkRole::Server => {
            network_handler.connection = Some(Connection::new_server("127.0.0.1:22022"));

            if let Some(mut connection) = network_handler.connection.take() {
                println!("{:?}", Start::try_from(&connection.read() as &[u8]));
            }
        }
        NetworkRole::Client => {
            network_handler.connection = Some(Connection::new_client("127.0.0.1:22022"));

            if let Some(mut connection) = network_handler.connection.take() {
                let start: Vec<u8> = chess_networking::Start {
                    is_white: false,
                    name: Some("Klientmannen".to_string()),
                    fen: None,
                    time: None,
                    inc: None,
                }
                .try_into()
                .unwrap();

                connection.write(start);
            }
        }
    }

    // camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 10.0, 8.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        },
        OnGameScreen,
    ));

    // lighting
    commands.spawn((
        DirectionalLightBundle {
            transform: Transform::from_xyz(15.0, 15.0, 15.0).looking_at(Vec3::ZERO, Vec3::Y),
            directional_light: DirectionalLight {
                illuminance: 1_500.,
                ..default()
            },
            ..default()
        },
        OnGameScreen,
    ));

    // board
    for x in 0..8 {
        for y in 0..8 {
            let offset = y % 2 == x % 2;
            commands
                .spawn((
                    PbrBundle {
                        mesh: meshes.add(Cuboid::new(1.0, 0.2, 1.0)),
                        material: if offset {
                            square_resource_data.white_square.clone()
                        } else {
                            square_resource_data.black_square.clone()
                        },
                        transform: Transform::from_xyz(
                            1.0 * (y as f32 - 3.5),
                            0.0,
                            1.0 * (x as f32 - 3.5),
                        ),
                        ..default()
                    },
                    OnGameScreen,
                ))
                .insert(PickableBundle::default())
                .insert(ChessSquare {
                    id: x * 8 + y,
                    offset,
                });
        }
    }

    // pieces
    for i in 0..64 {
        if let Some(piece) = game_state.board_state.piece_on(i) {
            board::spawn_piece(
                &mut commands,
                &piece_model_data,
                piece.t,
                piece.color,
                board_id_to_world_pos(i),
                &mut game_state,
            );
        }
    }
}
