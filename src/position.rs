use crate::bitboard::set_bit;
use crate::defs::{Bitboard, CastlingRights, Piece, Side, Square};
use std::collections::HashMap;

#[derive(PartialEq, Debug)]
pub struct Position {
    pub bitboards: [u64; 15],
    pub en_passant_square: u8,
    pub castling_rights: CastlingRights,
    pub side_to_move: Side,
    pub fifty_move_count: u16,
    pub halfmove_count: u16,
}

impl Position {
    pub fn default() -> Self {
        Self {
            bitboards: [0; 15],
            en_passant_square: Square::NoSquare as u8,
            castling_rights: 0b1111,
            side_to_move: Side::White,
            fifty_move_count: 0,
            halfmove_count: 2,
        }
    }

    pub fn from_position(position: HashMap<Square, Piece>) -> Self {
        let mut bitboards = [0; 15];

        for (square, piece) in position {
            set_bit(&mut bitboards[piece as usize], square as u8);

            let side_index = Self::get_side_index(piece);
            set_bit(&mut bitboards[side_index], square as u8);

            set_bit(&mut bitboards[Bitboard::ALL_PIECES], square as u8);
        }

        // TODO: initialize en passant square and castling rights

        Self {
            bitboards,
            en_passant_square: Square::NoSquare as u8,
            castling_rights: 0b1111,
            side_to_move: Side::White,
            fifty_move_count: 0,
            halfmove_count: 2,
        }
    }

    fn get_side_index(piece: Piece) -> usize {
        match Side::from(piece) {
            Side::White => Bitboard::WHITE_PIECES,
            Side::Black => Bitboard::BLACK_PIECES,
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

        verify_pos_bitboard(&position, Bitboard::WHITE_KING, vec![E1]);
        verify_pos_bitboard(&position, Bitboard::WHITE_PAWNS, vec![A2, B2]);
        verify_pos_bitboard(&position, Bitboard::WHITE_PIECES, vec![E1, A2, B2]);
        verify_pos_bitboard(&position, Bitboard::BLACK_KING, vec![E8]);
        verify_pos_bitboard(&position, Bitboard::BLACK_PIECES, vec![E8]);
        verify_pos_bitboard(&position, Bitboard::ALL_PIECES, vec![E1, A2, B2, E8]);
    }

    #[test]
    fn gets_side_index() {
        assert_eq!(Position::get_side_index(Piece::WhiteKing), Bitboard::WHITE_PIECES);
        assert_eq!(Position::get_side_index(Piece::BlackKing), Bitboard::BLACK_PIECES);
    }
}
