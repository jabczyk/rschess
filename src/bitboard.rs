pub fn get_bit(bitboard: u64, square: u8) -> u64 {
    bitboard & (1u64 << square)
}

pub fn set_bit(bitboard: &mut u64, square: u8) {
    *bitboard |= 1u64 << square
}

pub fn del_bit(bitboard: &mut u64, square: u8) {
    if get_bit(*bitboard, square) != 0 {
        *bitboard ^= 1u64 << square
    }
}

pub fn print_bitboard(bitboard: u64) {
    println!("   a b c d e f g h");
    for rank in 0..8 {
        print!("{} ", 8 - rank);

        for file in 0..8 {
            let square = rank * 8 + file;
            print!(" {}", if get_bit(bitboard, square) == 0 { 0 } else { 1 });
        }

        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_gets_bit() {
        assert_eq!(get_bit(5u64, 0), 1);
        assert_eq!(get_bit(5u64, 1), 0);
        assert_eq!(get_bit(5u64, 2), 4);
    }

    #[test]
    fn it_sets_bit() {
        let mut bitboard: u64 = 0;
        set_bit(&mut bitboard, 2);
        set_bit(&mut bitboard, 5);
        set_bit(&mut bitboard, 63);
        assert_eq!(get_bit(bitboard, 2), u64::pow(2, 2));
        assert_eq!(get_bit(bitboard, 5), u64::pow(2, 5));
        assert_eq!(get_bit(bitboard, 63), u64::pow(2, 63));
    }

    #[test]
    fn it_deletes_bit_if_necessary() {
        let mut bitboard: u64 = 4;
        del_bit(&mut bitboard, 2);
        assert_eq!(get_bit(bitboard, 2), 0);
        del_bit(&mut bitboard, 2);
        assert_eq!(get_bit(bitboard, 2), 0);
    }
}
