use std::f32::consts::PI;

use bevy::prelude::Color;
use bevy::prelude::*;
use bevy_mod_outline::*;
use bevy_mod_picking::*;
use events::{Click, Pointer};
use vhultman_chess::Color as PieceColor;
use vhultman_chess::*;

#[derive(Resource)]
struct PieceModelData {
    pawn_parts: Vec<Handle<Mesh>>,
    rook_parts: Vec<Handle<Mesh>>,
    knight_parts: Vec<Handle<Mesh>>,
    bishop_parts: Vec<Handle<Mesh>>,
    queen_parts: Vec<Handle<Mesh>>,
    king_parts: Vec<Handle<Mesh>>,
    white_material: Handle<StandardMaterial>,
    black_material: Handle<StandardMaterial>,
}

#[derive(Resource)]
struct SquareResourceData {
    white_square: Handle<StandardMaterial>,
    black_square: Handle<StandardMaterial>,
    selected_square: Handle<StandardMaterial>,
}

#[derive(Resource)]
struct GameState {
    board_state: Position,
    selected_piece: Option<usize>,
}

#[derive(Component)]
struct ChessPiece {
    _piece_type: PieceType,
    _color: PieceColor,
    id: usize,
}

#[derive(Component)]
struct ChessPiecePart;

#[derive(Component)]
struct ChessSquare {
    id: u32,
    offset: bool,
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, OutlinePlugin, DefaultPickingPlugins))
        .add_systems(Startup, load_resources)
        .add_systems(PostStartup, setup)
        .add_systems(Update, handle_picking)
        .run();
}

fn handle_picking(
    mut commands: Commands,
    mut events: EventReader<Pointer<Click>>,
    mut query: Query<(&Parent, &mut OutlineVolume), With<ChessPiecePart>>,
    mut piece_query: Query<(&mut Transform, &ChessPiece, &Children)>,
    mut tile_query: Query<(&mut Handle<StandardMaterial>, &ChessSquare)>,
    mut game_state: ResMut<GameState>,
    square_resource_data: Res<SquareResourceData>,
) {
    // Handle selection and deselection
    for ev in events.read() {
        if let Ok((parent, _)) = query.get_mut(ev.target) {
            let parent_entity = commands.entity(**parent);

            // set the color
            if let Ok((_, chess_piece, _)) = piece_query.get_mut(parent_entity.id()) {
                if game_state.selected_piece == Some(chess_piece.id) {
                    game_state.selected_piece = None;
                } else {
                    game_state.selected_piece = Some(chess_piece.id);
                }
            }
        }

        // Update color etc. for all pieces after the change
        let mut selected_translation: Option<Vec3> = None;
        for (transform, piece, children) in piece_query.iter_mut() {
            let selected = if let Some(selected_piece) = game_state.selected_piece {
                piece.id == selected_piece
            } else {
                false
            };

            if selected {
                selected_translation = Some(transform.translation);
            }

            for child in children.iter() {
                if let Ok(mut lol) = query.get_mut(*child) {
                    lol.1.colour = if selected {
                        Color::srgb(0.0, 1.0, 1.0)
                    } else {
                        Color::srgb(1.0, 1.0, 1.0)
                    };
                }
            }
        }

        let selected_square: Option<u32> = selected_translation
            .map(|translation| ((translation.z + 3.5) * 8.0 + translation.x + 3.5) as u32);

        let possible_moves: Vec<u32> = selected_square.map_or_else(Vec::new, |square| {
            game_state
                .board_state
                .moves_for_square(square)
                .iter()
                .map(|m| m.to())
                .collect()
        });

        for (mut material, square) in tile_query.iter_mut() {
            let possible = possible_moves.contains(&square.id);

            *material = if possible {
                square_resource_data.selected_square.clone()
            } else if square.offset {
                square_resource_data.white_square.clone()
            } else {
                square_resource_data.black_square.clone()
            };
        }
    }
}

fn spawn_piece(
    commands: &mut Commands,
    piece_model_data: &PieceModelData,
    piece_type: PieceType,
    color: PieceColor,
    position: Vec3,
    id: usize,
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
        .spawn((SpatialBundle {
            transform: Transform {
                translation: position,
                scale,
                rotation,
            },
            ..Default::default()
        },))
        .insert(PickableBundle::default())
        .insert(ChessPiece {
            _piece_type: piece_type,
            _color: color,
            id,
        })
        .id();

    for part in parts.iter() {
        let child = commands
            .spawn((PbrBundle {
                mesh: part.clone(),
                material: material.clone(),
                ..Default::default()
            },))
            .insert(OutlineBundle {
                outline: OutlineVolume {
                    visible: true,
                    colour: Color::srgb(1.0, 1.0, 1.0),
                    width: 1.5,
                },
                mode: OutlineMode::RealVertex,
                ..Default::default()
            })
            .insert(ChessPiecePart)
            .id();

        commands.entity(parent).push_children(&[child]);
    }
}

fn load_resources(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // piece meshes
    let knight: Handle<Mesh> = asset_server.load("chess_pieces.glb#Mesh0/Primitive0");
    let queen: Handle<Mesh> = asset_server.load("chess_pieces.glb#Mesh1/Primitive0");
    let king: Handle<Mesh> = asset_server.load("chess_pieces.glb#Mesh2/Primitive0");
    let pawn: Handle<Mesh> = asset_server.load("chess_pieces.glb#Mesh3/Primitive0");
    let bishop_p1: Handle<Mesh> = asset_server.load("chess_pieces.glb#Mesh4/Primitive0");
    let bishop_p2: Handle<Mesh> = asset_server.load("chess_pieces.glb#Mesh5/Primitive0");
    let rook_p1: Handle<Mesh> = asset_server.load("chess_pieces.glb#Mesh6/Primitive0");
    let rook_p2: Handle<Mesh> = asset_server.load("chess_pieces.glb#Mesh7/Primitive0");

    // piece color materials
    let white_material = materials.add(StandardMaterial {
        base_color: Color::srgb(1.0, 1.0, 1.0),
        metallic: 0.2,
        reflectance: 1.0,
        ..Default::default()
    });
    let black_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.0, 0.0, 0.0),
        metallic: 0.2,
        reflectance: 1.0,
        ..Default::default()
    });

    // Setup piece resources
    commands.insert_resource(PieceModelData {
        pawn_parts: vec![pawn],
        knight_parts: vec![knight],
        bishop_parts: vec![bishop_p1, bishop_p2],
        queen_parts: vec![queen],
        king_parts: vec![king],
        rook_parts: vec![rook_p1, rook_p2],
        white_material,
        black_material,
    });

    // Setup square resources
    commands.insert_resource(SquareResourceData {
        white_square: materials.add(Color::srgb_u8(255, 255, 255)),
        black_square: materials.add(Color::srgb_u8(0, 0, 0)),
        selected_square: materials.add(StandardMaterial {
            base_color: Color::srgb_u8(0, 255, 255),
            unlit: true,
            ..Default::default()
        }),
    });

    // Setup game state and more
    commands.insert_resource(GameState {
        board_state: Position::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR").unwrap(),
        selected_piece: None,
    });
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    piece_model_data: Res<PieceModelData>,
    square_resource_data: Res<SquareResourceData>,
    game_state: Res<GameState>,
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
                .insert(ChessSquare {
                    id: x * 8 + y,
                    offset,
                });
        }
    }

    let mut curr_id = 0;
    for y in 0..8 {
        for x in 0..8 {
            if let Some(piece) = game_state.board_state.piece_on(y * 8 + x) {
                spawn_piece(
                    &mut commands,
                    &piece_model_data,
                    piece.t,
                    piece.color,
                    Vec3::new(-3.5 + x as f32, 0.1, -2.5 - 1.0 + y as f32),
                    curr_id,
                );

                curr_id += 1;
            }
        }
    }
}
