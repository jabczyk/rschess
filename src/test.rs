use crate::defs::Square;
use crate::position::Position;

pub fn bitboard_with(squares: Vec<Square>) -> u64 {
    squares.iter().fold(0, |bitboard, square| bitboard + (1u64 << square.to_owned() as u64))
}

pub fn verify_bitboard(bitboard: u64, squares: Vec<Square>) {
    assert_eq!(bitboard, bitboard_with(squares))
}

pub fn verify_pos_bitboard(position: &Position, bitboard: usize, squares: Vec<Square>) {
    verify_bitboard(position.bitboards[bitboard], squares)
}
