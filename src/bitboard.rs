#[inline]
pub fn get_bit(bitboard: u64, square: u8) -> u64 {
    bitboard & bit_from_sq(square)
}

#[inline]
pub fn set_bit(bitboard: &mut u64, square: u8) {
    *bitboard |= 1u64 << square
}

#[inline]
pub fn del_bit(bitboard: &mut u64, square: u8) {
    if get_bit(*bitboard, square) != 0 {
        *bitboard ^= 1u64 << square
    }
}

#[inline]
pub fn sq(rank: u8, file: u8) -> u8 {
    rank * 8 + file
}

#[inline]
pub fn coords(square: u8) -> (u8, u8) {
    // (rank, file)
    (square / 8, square % 8)
}

#[inline]
pub fn bit_from_sq(square: u8) -> u64 {
    1 << square
}

// Gets an index of the least significant 1st bit of a bitboard
//
// https://www.chessprogramming.org/BitScan
#[inline]
pub fn get_ls1b_index(bitboard: u64) -> u8 {
    assert_ne!(bitboard, 0);
    bitboard.trailing_zeros() as u8
}

pub fn print_bitboard(bitboard: u64) {
    println!("   a b c d e f g h");
    for rank in 0..8 {
        print!("{} ", 8 - rank);

        for file in 0..8 {
            print!(" {}", if get_bit(bitboard, sq(rank, file)) == 0 { '.' } else { '#' });
        }

        println!();
    }
    println!("Bitboard: {}", bitboard);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::enums::Square::*;

    #[test]
    fn gets_bit() {
        assert_eq!(get_bit(5u64, 0), 1);
        assert_eq!(get_bit(5u64, 1), 0);
        assert_eq!(get_bit(5u64, 2), 4);
    }

    #[test]
    fn sets_bit() {
        let mut bitboard: u64 = 0;
        set_bit(&mut bitboard, 2);
        set_bit(&mut bitboard, 5);
        set_bit(&mut bitboard, 63);
        assert_eq!(get_bit(bitboard, 2), u64::pow(2, 2));
        assert_eq!(get_bit(bitboard, 5), u64::pow(2, 5));
        assert_eq!(get_bit(bitboard, 63), u64::pow(2, 63));
    }

    #[test]
    fn deletes_bit_if_necessary() {
        let mut bitboard: u64 = 4;
        del_bit(&mut bitboard, 2);
        assert_eq!(get_bit(bitboard, 2), 0);
        del_bit(&mut bitboard, 2);
        assert_eq!(get_bit(bitboard, 2), 0);
    }

    #[test]
    fn returns_square_index() {
        assert_eq!(sq(1, 0), 8);
        assert_eq!(sq(1, 2), 10);
        assert_eq!(sq(0, 7), 7);
    }

    #[test]
    fn gets_square_coords() {
        assert_eq!(coords(sq(1, 0)), (1, 0));
        assert_eq!(coords(sq(1, 2)), (1, 2));
        assert_eq!(coords(sq(0, 7)), (0, 7));
    }

    #[test]
    fn gets_bit_from_square() {
        assert_eq!(bit_from_sq(0), 1);
        assert_eq!(bit_from_sq(5), u64::pow(2, 5));
        assert_eq!(bit_from_sq(63), u64::pow(2, 63));
    }

    #[test]
    fn gets_ls1b_index() {
        assert_eq!(get_ls1b_index(bit_from_sq(A1 as u8)), A1 as u8);
        assert_eq!(get_ls1b_index(bit_from_sq(E4 as u8)), E4 as u8);
        assert_eq!(get_ls1b_index(bit_from_sq(H8 as u8)), H8 as u8);
    }

    #[test]
    #[should_panic]
    fn get_ls1b_panics_on_empty_board() {
        get_ls1b_index(0);
    }
}
