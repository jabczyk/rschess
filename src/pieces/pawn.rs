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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::enums::Square::*;
    use crate::test::*;

    mod get_attacks {
        use super::*;

        #[test]
        fn generates_attacks() {
            verify_bitboard(get_pawn_attacks(Side::White, E4 as u8), vec![D5, F5]);
            verify_bitboard(get_pawn_attacks(Side::Black, E5 as u8), vec![D4, F4]);
        }

        #[test]
        fn valid_a_h_files() {
            verify_bitboard(get_pawn_attacks(Side::White, A2 as u8), vec![B3]);
            verify_bitboard(get_pawn_attacks(Side::White, H2 as u8), vec![G3]);

            verify_bitboard(get_pawn_attacks(Side::Black, A7 as u8), vec![B6]);
            verify_bitboard(get_pawn_attacks(Side::Black, H7 as u8), vec![G6]);
        }

        #[test]
        fn edges() {
            verify_bitboard(get_pawn_attacks(Side::White, A8 as u8), vec![]);
            verify_bitboard(get_pawn_attacks(Side::White, H8 as u8), vec![]);
            verify_bitboard(get_pawn_attacks(Side::Black, A1 as u8), vec![]);
            verify_bitboard(get_pawn_attacks(Side::Black, H1 as u8), vec![]);
        }
    }

    #[test]
    fn generates_table() {
        let table = get_pawn_attacks_table();

        verify_bitboard(table[Side::White as usize][E4 as usize], vec![D5, F5]);
        verify_bitboard(table[Side::Black as usize][E5 as usize], vec![D4, F4]);
    }
}
