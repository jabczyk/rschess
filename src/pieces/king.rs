use crate::bitboard::*;
use crate::constants::{A_FILE, H_FILE};

pub fn get_king_attacks(square: u8) -> u64 {
    let mut attacks: u64 = 0;
    let mut king: u64 = 0;
    set_bit(&mut king, square);

    attacks |= king >> (8 - 1) & !A_FILE;
    attacks |= king >> 8;
    attacks |= king >> (8 + 1) & !H_FILE;
    attacks |= king >> 1 & !H_FILE;
    attacks |= king << 1 & !A_FILE;
    attacks |= king << (8 - 1) & !H_FILE;
    attacks |= king << 8;
    attacks |= king << (8 + 1) & !A_FILE;

    attacks
}

pub fn get_king_attacks_table() -> [u64; 64] {
    let mut table = [0; 64];

    for (square, board) in table.iter_mut().enumerate() {
        *board = get_king_attacks(square as u8);
    }

    table
}
