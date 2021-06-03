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
