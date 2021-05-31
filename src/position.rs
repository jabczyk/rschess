use crate::bitboard::set_bit;
use crate::enums::{Piece, Side, Square};
use std::collections::HashMap;

pub enum Bitboard {
    WhiteKing,
    WhiteQueens,
    WhiteRooks,
    WhiteBishops,
    WhiteKnights,
    WhitePawns,
    BlackKing,
    BlackQueens,
    BlackRooks,
    BlackBishops,
    BlackKnights,
    BlackPawns,
    WhitePieces,
    BlackPieces,
}

pub struct Position {
    pub bitboards: [u64; 14],
}

impl Position {
    pub fn default() -> Self {
        Self { bitboards: [0; 14] }
    }

    pub fn from_position(position: HashMap<Square, Piece>) -> Self {
        let mut bitboards = [0; 14];

        for (square, piece) in position {
            set_bit(&mut bitboards[piece as usize], square as u8);

            let side_index = Self::get_side_index(piece);
            set_bit(&mut bitboards[side_index], square as u8);
        }

        Self { bitboards }
    }

    fn get_side_index(piece: Piece) -> usize {
        match Side::from(piece) {
            Side::White => Bitboard::WhitePieces as usize,
            Side::Black => Bitboard::BlackPieces as usize,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::*;
    use Square::*;

    #[test]
    fn sets_custom_position() {
        let position = Position::from_position(map! {
            E1 => Piece::WhiteKing,
            A2 => Piece::WhitePawn,
            B2 => Piece::WhitePawn,
            E8 => Piece::BlackKing
        });

        verify_bitboard(&position, Bitboard::WhiteKing, vec![E1]);
        verify_bitboard(&position, Bitboard::WhitePawns, vec![A2, B2]);
        verify_bitboard(&position, Bitboard::WhitePieces, vec![E1, A2, B2]);
        verify_bitboard(&position, Bitboard::BlackKing, vec![E8]);
        verify_bitboard(&position, Bitboard::BlackPieces, vec![E8]);
    }

    #[test]
    fn gets_side_index() {
        assert_eq!(Position::get_side_index(Piece::WhiteKing), Bitboard::WhitePieces as usize);
        assert_eq!(Position::get_side_index(Piece::BlackKing), Bitboard::BlackPieces as usize);
    }
}
