use std::f32::consts::PI;

use bevy::prelude::Color;
use bevy::prelude::*;
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

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, load_resources)
        .add_systems(PostStartup, setup)
        .run();
}

fn spawn_piece(
    commands: &mut Commands,
    piece_model_data: &PieceModelData,
    piece_type: PieceType,
    color: PieceColor,
    position: Vec3,
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
        .spawn(SpatialBundle {
            transform: Transform {
                translation: position,
                scale,
                rotation,
                ..Default::default()
            },
            ..Default::default()
        })
        .id();

    for part in parts.iter() {
        let child = commands
            .spawn(PbrBundle {
                mesh: part.clone(),
                material: material.clone(),
                ..Default::default()
            })
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
        ..Default::default()
    });
    let black_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.0, 0.0, 0.0),
        ..Default::default()
    });

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
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    piece_model_data: Res<PieceModelData>,
) {
    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 12.0, 12.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });

    // lighting
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            intensity: 2_500_000.0,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    // board
    let white = Color::srgb_u8(255, 255, 255);
    let black = Color::srgb_u8(0, 0, 0);
    for x in 0..8 {
        for y in 0..8 {
            let offset = y % 2 == x % 2;
            commands.spawn(PbrBundle {
                mesh: meshes.add(Cuboid::new(1.0, 0.2, 1.0)),
                material: materials.add(if offset { white } else { black }),
                transform: Transform::from_xyz(1.0 * (x as f32 - 3.5), 0.0, 1.0 * (y as f32 - 3.5)),
                ..default()
            });
        }
    }

    let game = Position::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR").unwrap();

    for y in 0..8 {
        for x in 0..8 {
            if let Some(piece) = game.piece_on(y * 8 + x) {
                spawn_piece(
                    &mut commands,
                    &piece_model_data,
                    piece.t,
                    piece.color,
                    Vec3::new(-3.5 + x as f32, 0.0, -2.5 - 1.0 + y as f32),
                );
            }
        }
    }
}
