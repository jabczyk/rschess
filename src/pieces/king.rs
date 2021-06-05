use crate::bitboard::*;
use crate::constants::{A_FILE, H_FILE};

pub fn get_king_attacks(square: u8) -> u64 {
    let mut attacks: u64 = 0;
    let king = bit_from_sq(square);

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::enums::Square::*;
    use crate::test::*;

    mod get_attacks {
        use super::*;

        #[test]
        fn generates_attacks() {
            verify_bitboard(get_king_attacks(E4 as u8), vec![D5, E5, F5, D4, F4, D3, E3, F3]);
            verify_bitboard(get_king_attacks(B2 as u8), vec![A3, B3, C3, A2, C2, A1, B1, C1]);
        }

        #[test]
        fn valid_edges() {
            verify_bitboard(get_king_attacks(A1 as u8), vec![A2, B2, B1]);
            verify_bitboard(get_king_attacks(A8 as u8), vec![A7, B7, B8]);
            verify_bitboard(get_king_attacks(H1 as u8), vec![G1, G2, H2]);
            verify_bitboard(get_king_attacks(H8 as u8), vec![G8, G7, H7]);
        }

        #[test]
        fn generates_table() {
            assert_eq!(get_king_attacks_table()[E4 as usize], get_king_attacks(E4 as u8));
        }
    }
}
