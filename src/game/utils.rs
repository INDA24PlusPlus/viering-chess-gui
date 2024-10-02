use bevy::math::Vec3;
use vhultman_chess::Position;
use vhultman_chess::{Color as PieceColor, PieceType};

use super::ClientGameState;

pub fn world_pos_to_board_id(world_pos: Vec3) -> u32 {
    ((world_pos.z + 3.5) * 8.0 + world_pos.x + 3.5) as u32
}

pub fn board_id_to_world_pos(board_id: u32) -> Vec3 {
    Vec3::new(
        (board_id % 8) as f32 - 3.5,
        0.1,
        (board_id / 8) as f32 - 3.5,
    )
}

pub fn from_fen_extended(fen: &str) -> ClientGameState {
    let segments: Vec<&str> = fen.split(" ").collect();

    if segments.len() != 6 {
        panic!("Bad fen");
    }

    // segment 1: board :D
    let game_pos: Position = Position::from_fen(segments[0]).expect("Failed to parse segment 1");

    // segment 2: turn
    // TODO CANT SET WHOS TURN IT IS, DEFAULTING TO WHITE :/

    // segment 3: castling ability
    // TODO CANT SET CASTLING ABILITY, DEFAULTING TO ALL AVAILABLE :(

    // segment 4: en passant target square
    // TODO CANT SET EN PASSANT TARGET SQUARE, DEFAULTING TO NONE :'(

    // segment 5: halfmove clock
    // TODO CANT SET MOVE, DEFAULTING TO NONE >:(

    // segment 6: fullmove counter (i will skip this)

    ClientGameState {
        board_state: game_pos,
        selected_piece: None,
        spawned_pieces: 0,
        board_dirty: true,
        last_move: None,
        pending_promotion_move: None,
    }
}

pub fn to_fen_extended(position: &Position) -> String {
    let mut segment_1 = String::new();

    for y in 0..8 {
        let mut empty_count = 0;
        for x in 0..8 {
            let piece = position.piece_on(y * 8 + x);
            if let Some(piece) = piece {
                if empty_count > 0 {
                    segment_1 = format!("{}{}", segment_1, empty_count);
                }

                let mut chr: String = match piece.t {
                    PieceType::Pawn => "p",
                    PieceType::Knight => "n",
                    PieceType::Bishop => "b",
                    PieceType::Rook => "r",
                    PieceType::Queen => "q",
                    PieceType::King => "k",
                }
                .to_string();

                if piece.color == PieceColor::White {
                    chr = chr.to_uppercase();
                }

                segment_1 = format!("{}{}", segment_1, chr);
            } else {
                empty_count += 1;
            }
        }

        if empty_count > 0 {
            segment_1 = format!("{}{}", segment_1, empty_count);
        }

        if y != 7 {
            segment_1 += "/";
        }
    }

    let segment_2 = if position.current_side() == PieceColor::White {
        "w"
    } else {
        "b"
    };

    let segment_3 = "KQkq"; // rip cant read, using default

    let segment_4 = "-"; // rip cant read, using default

    let segment_5 = "0"; // rip cant read, using default

    let segment_6 = "1"; // rip cant read, using default

    format!(
        "{} {} {} {} {} {}",
        segment_1, segment_2, segment_3, segment_4, segment_5, segment_6
    )
}
