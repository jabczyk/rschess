use crate::{
    bitboard::*,
    enums::{Side, Square},
    position::{Bitboard, Position},
};

impl Position {
    pub fn from_fen(fen: &str) -> Self {
        let fen_parts: Vec<&str> = fen.split_whitespace().collect();

        Self {
            bitboards: place_pieces(fen_parts[0]),
            en_passant_square: Square::NoSquare,
            castling_rights: 0b1111,
            side_to_move: Side::White,
            fifty_move_count: 0,
            halfmove_count: 0,
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
