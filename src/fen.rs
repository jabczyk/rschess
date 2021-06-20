use crate::{
    bitboard::*,
    enums::{Castling, CastlingRights, Side, Square},
    position::{Bitboard, Position},
};

impl Position {
    // Creates a position from [FEN] string
    //
    // [FEN]: https://www.chessprogramming.org/Forsyth-Edwards_Notation
    pub fn from_fen(fen: &str) -> Self {
        let fen_parts: Vec<&str> = fen.split_whitespace().collect();
        assert_eq!(fen_parts.len(), 6);

        Self {
            bitboards: place_pieces(fen_parts[0]),
            en_passant_square: Square::NoSquare,
            castling_rights: get_castling_rights(fen_parts[2]),
            side_to_move: fen_parts[1].into(),
            fifty_move_count: fen_parts[4].parse().unwrap(),
            halfmove_count: get_halfmoves(fen_parts[5].parse().unwrap(), fen_parts[1].into()),
        }
    }
}

// Takes piece placement part of a FEN string, returns an array of bitboards
// representing the placement
fn place_pieces(placement: &str) -> [u64; 15] {
    let mut bitboards = [0u64; 15];
    let (mut rank, mut file) = (0u8, 0u8);

    for c in placement.chars() {
        let mut set_piece = |board: Bitboard| {
            set_bit(&mut bitboards[board as usize], sq(rank, file));
            file += 1;
        };

        match c {
            'K' => set_piece(Bitboard::WhiteKing),
            'Q' => set_piece(Bitboard::WhiteQueens),
            'R' => set_piece(Bitboard::WhiteRooks),
            'B' => set_piece(Bitboard::WhiteBishops),
            'N' => set_piece(Bitboard::WhiteKnights),
            'P' => set_piece(Bitboard::WhitePawns),
            'k' => set_piece(Bitboard::BlackKing),
            'q' => set_piece(Bitboard::BlackQueens),
            'r' => set_piece(Bitboard::BlackRooks),
            'b' => set_piece(Bitboard::BlackBishops),
            'n' => set_piece(Bitboard::BlackKnights),
            'p' => set_piece(Bitboard::BlackPawns),
            '1'..='8' => file += c as u8 - 0x30,
            '/' => {
                rank += 1;
                file = 0;
            }
            _ => (),
        }
    }

    bitboards[Bitboard::WhitePieces as usize] = sum_bitboards(&bitboards[0..6]);
    bitboards[Bitboard::BlackPieces as usize] = sum_bitboards(&bitboards[6..12]);
    bitboards[Bitboard::AllPieces as usize] =
        bitboards[Bitboard::WhitePieces as usize] | bitboards[Bitboard::BlackPieces as usize];

    bitboards
}

// Parses FEN castling rights string (like "KQkq"), returns castling right integer
fn get_castling_rights(rights_str: &str) -> CastlingRights {
    let mut rights = 0;

    for c in rights_str.chars() {
        match c {
            'K' => rights |= Castling::WK as u8,
            'Q' => rights |= Castling::WQ as u8,
            'k' => rights |= Castling::BK as u8,
            'q' => rights |= Castling::BQ as u8,
            _ => break,
        }
    }

    rights
}

fn get_halfmoves(full_moves: u16, side_to_move: Side) -> u16 {
    (full_moves * 2) + if side_to_move == Side::Black { 1 } else { 0 }
}
