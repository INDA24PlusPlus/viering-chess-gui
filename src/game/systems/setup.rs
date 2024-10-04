use std::time::Duration;

use bevy::prelude::*;
use bevy_mod_picking::PickableBundle;
use vhultman_chess::Color as PieceColor;
use vhultman_chess::Position;

use crate::game::NetworkState;
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

            if let Some(connection) = network_handler.connection.as_mut() {
                let packet = chess_networking::Start::try_from(&connection.read() as &[u8])
                    .expect("Bad packet");

                println!(
                    "Client with name {} connected",
                    packet.name.unwrap_or("client".to_string())
                );

                let response_packet = chess_networking::Start {
                    is_white: true,
                    name: Some("Servermannen".to_string()),
                    fen: Some(
                        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string(),
                    ),
                    time: None,
                    inc: None,
                };

                let response_packet_bytes: Vec<u8> = response_packet.clone().try_into().unwrap();

                *game_state = ClientGameState {
                    board_state: Position::from_fen(response_packet.fen.unwrap().as_str())
                        .expect("Failed to parse initial server fen string"),
                    board_dirty: true,
                    last_move: None,
                    pending_promotion_move: None,
                    selected_piece: None,
                    spawned_pieces: 0,
                    own_color: if response_packet.is_white {
                        PieceColor::White
                    } else {
                        PieceColor::Black
                    },
                    network_state: NetworkState::Normal,
                    next_ack_state: None,
                };

                connection.write(response_packet_bytes);
            }
        }
        NetworkRole::Client => {
            let mut address = "127.0.0.1:22022";
            if let Some(addr) = &network_handler.address_to_join {
                if !addr.is_empty() {
                    address = addr;
                }
            }

            network_handler.connection = Some(Connection::new_client(address));

            if let Some(connection) = network_handler.connection.as_mut() {
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

                // wait for start packet from server
                std::thread::sleep(Duration::from_secs(2));
                let buf: Vec<u8>;
                loop {
                    let new_buf = connection.read();
                    if !new_buf.is_empty() {
                        buf = new_buf;
                        break;
                    }
                    std::thread::sleep(Duration::from_secs(1));
                }

                let packet = chess_networking::Start::try_from(&buf as &[u8]).expect("Bad packet");

                *game_state = ClientGameState {
                    board_state: Position::from_fen(
                        packet
                            .fen
                            .unwrap_or("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR".to_string())
                            .as_str(),
                    )
                    .expect("Failed to parse initial server fen string"),
                    board_dirty: true,
                    last_move: None,
                    pending_promotion_move: None,
                    selected_piece: None,
                    spawned_pieces: 0,
                    own_color: if packet.is_white {
                        PieceColor::Black
                    } else {
                        PieceColor::White
                    },
                    network_state: NetworkState::Normal,
                    next_ack_state: None,
                };
            }
        }
    }

    game_state.network_state = if game_state.board_state.current_side() == game_state.own_color {
        NetworkState::Normal
    } else {
        NetworkState::AwaitingMove
    };

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
