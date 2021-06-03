use crate::bitboard::*;

// Creates relevant bishop occupancy to use as a magic bitboard key
// Returns a bitboard with all possible bishop moves, on an empty board,
// excluding the board edges
//
// https://www.chessprogramming.org/Magic_Bitboards#How_it_works
pub fn get_bishop_occupancy(square: u8) -> u64 {
    let (b_rank, b_file) = coords(square);
    let mut bitboard: u64 = 0;

    let directions = [(1, 1), (-1, 1), (1, -1), (-1, -1)];
    for (dir_rank, dir_file) in &directions {
        let mut rank = b_rank as i8 + dir_rank;
        let mut file = b_file as i8 + dir_file;

        let in_bounds = |dir: i8, val: i8| if dir == 1 { val < 8 - 1 } else { val >= 1 };

        while in_bounds(*dir_rank, rank) && in_bounds(*dir_file, file) {
            set_bit(&mut bitboard, sq(rank as u8, file as u8));

            rank += dir_rank;
            file += dir_file;
        }
    }

    bitboard
}
