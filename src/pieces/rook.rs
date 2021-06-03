use crate::bitboard::*;

// Creates relevant rook occupancy to use as a magic bitboard key
// Returns a bitboard with all possible rook moves, on an empty board,
// excluding the board edges
//
// https://www.chessprogramming.org/Magic_Bitboards#How_it_works
pub fn get_rook_occupancy(square: u8) -> u64 {
    let (r_rank, r_file) = coords(square);
    let mut bitboard: u64 = 0;

    for rank in (r_rank + 1)..(8 - 1) {
        set_bit(&mut bitboard, sq(rank, r_file));
    }

    for rank in 1..r_rank {
        set_bit(&mut bitboard, sq(rank, r_file));
    }

    for file in (r_file + 1)..(8 - 1) {
        set_bit(&mut bitboard, sq(r_rank, file));
    }

    for file in 1..r_file {
        set_bit(&mut bitboard, sq(r_rank, file));
    }

    bitboard
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::enums::Square::*;
    use crate::test::*;

    mod get_occupancy {
        use super::*;

        #[test]
        fn generates_occupacy() {
            verify_bitboard(
                get_rook_occupancy(E4 as u8),
                vec![E7, E6, E5, E3, E2, B4, C4, D4, F4, G4],
            );
        }

        #[test]
        fn valid_edge_a1() {
            verify_bitboard(
                get_rook_occupancy(A1 as u8),
                vec![A7, A6, A5, A4, A3, A2, B1, C1, D1, E1, F1, G1],
            );
        }

        #[test]
        fn valid_edge_a8() {
            verify_bitboard(
                get_rook_occupancy(A8 as u8),
                vec![B8, C8, D8, E8, F8, G8, A7, A6, A5, A4, A3, A2],
            );
        }

        #[test]
        fn valid_edge_h1() {
            verify_bitboard(
                get_rook_occupancy(H1 as u8),
                vec![H7, H6, H5, H4, H3, H2, B1, C1, D1, E1, F1, G1],
            );
        }

        #[test]
        fn valid_edge_h8() {
            verify_bitboard(
                get_rook_occupancy(H8 as u8),
                vec![B8, C8, D8, E8, F8, G8, H7, H6, H5, H4, H3, H2],
            );
        }
    }
}
