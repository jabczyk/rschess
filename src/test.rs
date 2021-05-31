use crate::enums::Square;
use crate::position::{Bitboard, Position};

fn bitboard_with(squares: Vec<Square>) -> u64 {
    squares.iter().fold(0, |bitboard, square| bitboard + (1u64 << square.to_owned() as u64))
}

pub fn verify_bitboard(position: &Position, bitboard: Bitboard, squares: Vec<Square>) {
    assert_eq!(position.bitboards[bitboard as usize], bitboard_with(squares))
}
