use crate::bitboard::*;

// Creates relevant bishop occupancy to use as a magic bitboard key
// Returns a bitboard with all possible bishop moves, on an empty board,
// excluding the board edges
//
// https://www.chessprogramming.org/Magic_Bitboards#How_it_works
pub fn get_bishop_occupancy(square: u8) -> u64 {
    #[rustfmt::skip]
    get_bishop_moves(
        square,
        0,
        &|dir: i8, val: i8| if dir == 1 { val < 8 - 1 } else { val >= 1 }
    )
}

// Generates a bitboard all possible bishop attacks, taking blocking
// pieces into account
// It is used to seed the attacks bitboard database
pub fn get_bishop_attacks(square: u8, blockers: u64) -> u64 {
    #[rustfmt::skip]
    get_bishop_moves(
        square,
        blockers,
        &|dir: i8, val: i8| if dir == 1 { val < 8 } else { val >= 0 }
    )
}

// Generates a bitboard of all possible bishop (diagonal) moves,
// taking blocking pieces and coordinate bounds into account
fn get_bishop_moves(square: u8, blockers: u64, in_bounds: &dyn Fn(i8, i8) -> bool) -> u64 {
    let (b_rank, b_file) = coords(square);
    let mut bitboard: u64 = 0;

    let directions = [(1, 1), (-1, 1), (1, -1), (-1, -1)];
    for (dir_rank, dir_file) in &directions {
        let mut rank = b_rank as i8 + dir_rank;
        let mut file = b_file as i8 + dir_file;

        while in_bounds(*dir_rank, rank) && in_bounds(*dir_file, file) {
            let attack_square = sq(rank as u8, file as u8);
            set_bit(&mut bitboard, attack_square);

            if get_bit(blockers, attack_square) != 0 {
                break;
            };

            rank += dir_rank;
            file += dir_file;
        }
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
                get_bishop_occupancy(E4 as u8),
                vec![B7, C6, G6, D5, F5, D3, F3, C2, G2],
            );
            verify_bitboard(get_bishop_occupancy(B3 as u8), vec![F7, E6, D5, C4, C2]);
        }

        #[test]
        fn valid_edges() {
            verify_bitboard(get_bishop_occupancy(A4 as u8), vec![D7, C6, B5, B3, C2]);
            assert_eq!(get_bishop_occupancy(H1 as u8), get_bishop_occupancy(A8 as u8));
        }
    }

    mod get_attacks {
        use super::*;

        #[test]
        fn generates_attacks() {
            verify_bitboard(
                get_bishop_attacks(D4 as u8, 0),
                vec![H8, A7, G7, B6, F6, C5, E5, C3, E3, B2, F2, A1, G1],
            );
        }

        #[test]
        fn valid_edges() {
            verify_bitboard(get_bishop_attacks(A4 as u8, 0), vec![E8, D7, C6, B5, B3, C2, D1]);
        }

        #[test]
        fn handles_blockers() {
            let blockers = bitboard_with(vec![F6, A7, B2, F2]);

            verify_bitboard(
                get_bishop_attacks(D4 as u8, blockers),
                vec![A7, B6, F6, C5, E5, C3, E3, B2, F2],
            );
        }
    }
}
