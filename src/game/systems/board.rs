use bevy::prelude::*;
use bevy_mod_outline::{OutlineBundle, OutlineMode, OutlineVolume};
use bevy_mod_picking::PickableBundle;
use chess_networking::PromotionPiece;
use vhultman_chess::{Color as PieceColor, GameState, Piece, PieceType};

use std::f32::consts::PI;

use bevy::prelude::Color;

use crate::game::{
    board_id_to_world_pos, world_pos_to_board_id, ChessPiece, ChessPiecePart, ClientGameState,
    NetworkState, OnGameScreen, PieceModelData,
};
use crate::general::resources::NetworkHandler;
use crate::SoundEffects;

pub(crate) fn spawn_piece(
    commands: &mut Commands,
    piece_model_data: &PieceModelData,
    piece_type: PieceType,
    color: PieceColor,
    position: Vec3,
    game_state: &mut ClientGameState,
) {
    let material = if color == PieceColor::White {
        piece_model_data.white_material.clone()
    } else {
        piece_model_data.black_material.clone()
    };

    let parts: &Vec<Handle<Mesh>>;
    let scale: Vec3;
    let rotation: Quat =
        Quat::from_rotation_y(-PI / 2.0 + if color == PieceColor::White { 0.0 } else { PI });

    match piece_type {
        PieceType::Pawn => {
            parts = &piece_model_data.pawn_parts;
            scale = Vec3::splat(0.24);
        }
        PieceType::Rook => {
            parts = &piece_model_data.rook_parts;
            scale = Vec3::splat(0.23);
        }
        PieceType::Knight => {
            parts = &piece_model_data.knight_parts;
            scale = Vec3::splat(0.32);
        }
        PieceType::Bishop => {
            parts = &piece_model_data.bishop_parts;
            scale = Vec3::splat(0.24);
        }
        PieceType::Queen => {
            parts = &piece_model_data.queen_parts;
            scale = Vec3::splat(0.23);
        }
        PieceType::King => {
            parts = &piece_model_data.king_parts;
            scale = Vec3::splat(0.18);
        }
    };

    let parent = commands
        .spawn((
            SpatialBundle {
                transform: Transform {
                    translation: position,
                    scale,
                    rotation,
                },
                ..Default::default()
            },
            OnGameScreen,
        ))
        .insert(PickableBundle::default())
        .insert(ChessPiece {
            piece: Piece {
                t: piece_type,
                color,
            },
            id: game_state.spawned_pieces,
        })
        .id();

    for part in parts.iter() {
        let child = commands
            .spawn((
                PbrBundle {
                    mesh: part.clone(),
                    material: material.clone(),
                    ..Default::default()
                },
                OnGameScreen,
            ))
            .insert((
                OutlineBundle {
                    outline: OutlineVolume {
                        visible: true,
                        colour: Color::srgb(0.0, 0.0, 0.0),
                        width: 2.0,
                    },
                    mode: OutlineMode::RealVertex,
                    ..Default::default()
                },
                OnGameScreen,
            ))
            .insert(ChessPiecePart)
            .id();

        commands.entity(parent).push_children(&[child]);
    }

    game_state.spawned_pieces += 1;
}

pub(crate) fn update_board(
    mut commands: Commands,
    mut piece_query: Query<(Entity, &mut Transform, &ChessPiece, &Children)>,
    mut game_state: ResMut<ClientGameState>,
    piece_model_data: Res<PieceModelData>,
    sound_effects: Res<SoundEffects>,
) {
    if !game_state.board_dirty {
        return;
    }

    game_state.board_state.check_game_state();

    if let Some(m) = game_state.last_move {
        if m.is_promotion() {
            spawn_piece(
                &mut commands,
                &piece_model_data,
                m.promotion_piece(),
                if game_state.board_state.current_side() == PieceColor::White {
                    PieceColor::Black
                } else {
                    PieceColor::White
                },
                board_id_to_world_pos(m.to()),
                &mut game_state,
            );
        }
    }

    game_state.board_dirty = false;

    let should_play_sound = game_state.last_move.is_some();

    let pieces: Vec<(Entity, Mut<'_, Transform>, &ChessPiece)> = piece_query
        .iter_mut()
        .map(|(entity, transform, piece, _)| (entity, transform, piece))
        .collect();

    let mut entities_to_despawn: Vec<Entity> = Vec::new();

    for (entity, mut transform, piece) in pieces {
        if let Some(last_move) = game_state.last_move {
            if transform.translation == board_id_to_world_pos(last_move.from()) {
                transform.translation = board_id_to_world_pos(last_move.to());
            }
        }

        let board_id = world_pos_to_board_id(transform.translation);

        if game_state
            .board_state
            .piece_on(board_id)
            .map_or(true, |correct_piece| {
                piece.piece.color != correct_piece.color || piece.piece.t != correct_piece.t
            })
        {
            // queue the entity for despawning
            entities_to_despawn.push(entity);
        }
    }

    // spawn sound for despawning or valid move
    if should_play_sound {
        commands.spawn(AudioBundle {
            source: if !entities_to_despawn.is_empty() {
                sound_effects.capture.clone()
            } else {
                sound_effects.valid_move.clone()
            },
            ..default()
        });
    }

    // despawn pieces
    for entity in entities_to_despawn {
        commands.entity(entity).despawn_recursive();
    }

    // spawn pieces that should exist but don't
    for i in 0..64 {
        if let Some(piece) = game_state.board_state.piece_on(i) {
            // Check if a piece already exists in the right spot
            let piece_exists = piece_query
                .iter()
                .any(|(_, transform, _, _)| world_pos_to_board_id(transform.translation) == i);

            if !piece_exists {
                spawn_piece(
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
}

pub(crate) fn wait_for_move(
    mut game_state: ResMut<ClientGameState>,
    mut network_handler: ResMut<NetworkHandler>,
) {
    if game_state.network_state == NetworkState::Normal {
        return;
    }

    //let role = network_handler.role;
    if let Some(connection) = network_handler.connection.as_mut() {
        // wait for start packet from server
        let buf: Vec<u8> = connection.read();
        if buf.is_empty() {
            return;
        }

        if game_state.network_state == NetworkState::AwaitingAck {
            let packet = chess_networking::Ack::try_from(&buf as &[u8]).expect("Bad packet");

            game_state.next_ack_state = packet.end_state;

            if packet.ok {
                game_state.network_state = NetworkState::AwaitingMove;
                println!("received ack packet, its ok! time to make a move for us!");
            } else {
                // an illegal move is supposed to make the server win (no matter who doesn't accept
                // the move), but endstate in the networking specs doesn't support setting who
                // checkmated who so im just setting checmkate for now no matter what in next ack packet
                // thus logic below is commented out

                //if role == NetworkRole::Server {
                //    // we just won, time to send out end state
                //} else {
                //    // we just lost, time to resign
                //}

                game_state.next_ack_state = Some(chess_networking::GameState::CheckMate);
            }

            return;
        }

        println!("{:?}", buf);
        let packet = chess_networking::Move::try_from(&buf as &[u8]).expect("Bad packet");

        let from_id = square_coords_to_id(packet.from);
        let to_id = square_coords_to_id(packet.to);

        let possible_moves: Vec<u32> = game_state
            .board_state
            .moves_for_square(from_id)
            .iter()
            .map(|m| m.to())
            .collect();

        let mut move_accepted = false;
        if possible_moves.contains(&to_id) {
            let mut possible_move = game_state.board_state.get_move(from_id, to_id);

            if let Some(m) = possible_move.as_mut() {
                if let Some(promotion_piece) = packet.promotion {
                    m.set_promotion_piece(match promotion_piece {
                        PromotionPiece::Rook => PieceType::Rook,
                        PromotionPiece::Knight => PieceType::Knight,
                        PromotionPiece::Bishop => PieceType::Bishop,
                        PromotionPiece::Queen => PieceType::Queen,
                    });
                }

                game_state.board_state.make_move(*m);
                game_state.last_move = Some(*m);
                move_accepted = true;
                game_state.board_dirty = true;

                if game_state.next_ack_state.is_none() {
                    if let GameState::Checkmate = game_state.board_state.check_game_state() {
                        game_state.next_ack_state = Some(chess_networking::GameState::CheckMate)
                    }
                };
            }
        }

        if move_accepted {
            game_state.network_state = NetworkState::Normal;
        }

        connection.write(
            (chess_networking::Ack {
                ok: move_accepted,
                end_state: game_state.next_ack_state.clone(),
            })
            .try_into()
            .unwrap(),
        );
    }
}

fn square_coords_to_id(coords: (u8, u8)) -> u32 {
    ((7 - coords.1) * 8 + coords.0).into()
}
