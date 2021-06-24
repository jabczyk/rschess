use crate::bitboard::*;
use crate::constants::{AB_FILE, A_FILE, GH_FILE, H_FILE};

pub fn get_knight_attacks(square: u8) -> u64 {
    let mut attacks: u64 = 0;
    let knight = bit_from_sq(square);

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::defs::Square::*;
    use crate::test::*;

    mod get_attacks {
        use super::*;

        #[test]
        fn generates_attacks() {
            verify_bitboard(get_knight_attacks(F3 as u8), vec![E5, G5, D4, H4, D2, H2, E1, G1]);
            verify_bitboard(get_knight_attacks(C6 as u8), vec![B8, D8, A7, E7, A5, E5, B4, D4]);
        }

        #[test]
        fn valid_edges() {
            verify_bitboard(get_knight_attacks(A6 as u8), vec![B8, C7, C5, B4]);
            verify_bitboard(get_knight_attacks(B6 as u8), vec![A8, C8, D7, D5, A4, C4]);
            verify_bitboard(get_knight_attacks(H3 as u8), vec![G5, F4, F2, G1]);
            verify_bitboard(get_knight_attacks(G3 as u8), vec![F5, H5, E4, E2, F1, H1]);
        }

        #[test]
        fn generates_table() {
            verify_bitboard(get_knight_attacks(F3 as u8), vec![E5, G5, D4, H4, D2, H2, E1, G1]);
        }
    }
}
