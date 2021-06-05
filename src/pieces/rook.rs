use crate::bitboard::*;
use crate::enums::Axis;

// Creates relevant rook occupancy to use as a magic bitboard key
// Returns a bitboard with all possible rook moves, on an empty board,
// excluding the board edges
//
// https://www.chessprogramming.org/Magic_Bitboards#How_it_works
pub fn get_rook_occupancy(square: u8) -> u64 {
    let (rank, file) = coords(square);
    let mut rook: u64 = 0;
    set_bit(&mut rook, square);

    let rank_moves = get_rook_moves(Axis::Rank, file, 1..(8 - 1), 0);
    let file_moves = get_rook_moves(Axis::File, rank, 1..(8 - 1), 0);

    (rank_moves | file_moves) & !rook
}

// Generates a bitboard of all possible rook attacks, taking blocking
// pieces into account
// It is used to seed the attacks bitboard database
pub fn get_rook_attacks(square: u8, blockers: u64) -> u64 {
    let (rank, file) = coords(square);

    get_rook_moves(Axis::Rank, file, (rank + 1)..8, blockers)
        | get_rook_moves(Axis::Rank, file, (0..rank).rev(), blockers)
        | get_rook_moves(Axis::File, rank, (file + 1)..8, blockers)
        | get_rook_moves(Axis::File, rank, (0..file).rev(), blockers)
}

// Creates a bitboard of all possible rook attacks on a certain axis,
// taking blocking pieces and coordinate ranges into account
fn get_rook_moves<R>(axis: Axis, static_coord: u8, range: R, blockers: u64) -> u64
where
    R: Iterator<Item = u8>,
{
    let mut bitboard: u64 = 0;

    for coord in range {
        let attack_square = match axis {
            Axis::Rank => sq(coord, static_coord),
            Axis::File => sq(static_coord, coord),
        };
        set_bit(&mut bitboard, attack_square);
        if get_bit(blockers, attack_square) != 0 {
            break;
        };
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
