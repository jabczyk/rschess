use crate::bitboard::*;
use crate::constants::{A_FILE, H_FILE};
use crate::enums::Side;

pub fn get_pawn_attacks(side: Side, square: u8) -> u64 {
    let mut pawn: u64 = 0;
    set_bit(&mut pawn, square);

    let mut attacks: u64 = 0;

    match side {
        Side::White => {
            let attack = pawn >> 8 - 1;
            if attack & !A_FILE != 0 {
                attacks |= attack
            };

            let attack = pawn >> 8 + 1;
            if attack & !H_FILE != 0 {
                attacks |= attack
            };
        }
        Side::Black => {
            let attack = pawn << 8 - 1;
            if attack & !H_FILE != 0 {
                attacks |= attack
            };

            let attack = pawn << 8 + 1;
            if attack & !A_FILE != 0 {
                attacks |= attack
            };
        }
    };

    attacks
}

pub fn get_pawn_attacks_table() -> [[u64; 64]; 2] {
    let mut table = [[0; 64]; 2];

    for square in 0..64 {
        table[Side::White as usize][square] = get_pawn_attacks(Side::White, square as u8);
        table[Side::Black as usize][square] = get_pawn_attacks(Side::Black, square as u8);
    }

    table
}
