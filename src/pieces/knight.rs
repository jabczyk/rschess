use crate::bitboard::*;
use crate::constants::{AB_FILE, A_FILE, GH_FILE, H_FILE};

pub fn get_knight_attacks(square: u8) -> u64 {
    let mut attacks: u64 = 0;
    let mut knight: u64 = 0;
    set_bit(&mut knight, square);

    attacks |= knight >> (2 * 8 + 1) & !H_FILE;
    attacks |= knight >> (2 * 8 - 1) & !A_FILE;
    attacks |= knight >> (8 + 2) & !GH_FILE;
    attacks |= knight >> (8 - 2) & !AB_FILE;
    attacks |= knight << (2 * 8 + 1) & !A_FILE;
    attacks |= knight << (2 * 8 - 1) & !H_FILE;
    attacks |= knight << (8 + 2) & !AB_FILE;
    attacks |= knight << (8 - 2) & !GH_FILE;

    attacks
}

pub fn get_knight_attacks_table() -> [u64; 64] {
    let mut table = [0; 64];

    for (square, board) in table.iter_mut().enumerate() {
        *board = get_knight_attacks(square as u8);
    }

    table
}
