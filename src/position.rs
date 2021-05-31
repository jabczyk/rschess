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

