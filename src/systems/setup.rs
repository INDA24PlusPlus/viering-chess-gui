use bevy::prelude::*;
use bevy_mod_picking::PickableBundle;

use crate::{ChessSquare, ClientGameState, PieceModelData, SquareResourceData};

use super::pieces;

pub(crate) fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    piece_model_data: Res<PieceModelData>,
    square_resource_data: Res<SquareResourceData>,
    mut game_state: ResMut<ClientGameState>,
) {
    // Setup scene
    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 10.0, 8.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });

    // lighting
    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_xyz(50.0, 50.0, 50.0).looking_at(Vec3::ZERO, Vec3::Y),
        directional_light: DirectionalLight {
            illuminance: 1_500.,
            ..default()
        },
        ..default()
    });

    // board
    for x in 0..8 {
        for y in 0..8 {
            let offset = y % 2 == x % 2;
            commands
                .spawn(PbrBundle {
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
                })
                .insert(PickableBundle::default())
                .insert(ChessSquare {
                    id: x * 8 + y,
                    offset,
                });
        }
    }

    for y in 0..8 {
        for x in 0..8 {
            if let Some(piece) = game_state.board_state.piece_on(y * 8 + x) {
                pieces::spawn_piece(
                    &mut commands,
                    &piece_model_data,
                    piece.t,
                    piece.color,
                    Vec3::new(-3.5 + x as f32, 0.1, -3.5 + y as f32),
                    &mut game_state,
                );
            }
        }
    }
}
