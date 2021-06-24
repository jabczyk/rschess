use crate::{
    bitboard::*,
    constants::{HUMAN_SQUARES, PIECE_CHARS},
    enums::{Castling, Square},
    position::Position,
};

impl Position {
    pub fn print_board(&self) {
        for rank in 0..8 {
            print!("{} ", rank);

            for file in 0..8 {
                let mut piece_index: i8 = -1;
                for piece in 0..12 {
                    if get_bit(self.bitboards[piece], sq(rank, file)) != 0 {
                        piece_index = piece as i8;
                        break;
                    }
                }

                let piece_char =
                    if piece_index != -1 { PIECE_CHARS[piece_index as usize] } else { '.' };
                print!(" {}", piece_char);
            }

            println!();
        }

        println!("\n   a b c d e f g h\n");
        println!("Fifty clock: {}  Moves: {}", self.fifty_move_count, self.halfmove_count / 2);
        println!("Castling rights: {}", castling_rights(self.castling_rights));
        println!("En passant: {}", en_passant_square(self.en_passant_square));
        println!("Side to move: {}", self.side_to_move);
    }
}

fn en_passant_square<'a>(square: u8) -> &'a str {
    if square != Square::NoSquare as u8 {
        HUMAN_SQUARES[square as usize]
    } else {
        "--"
    }
}

fn castling_rights(rights: u8) -> String {
    format!(
        "{}{}{}{}",
        if rights & Castling::WK as u8 != 0 { "K" } else { "-" },
        if rights & Castling::WQ as u8 != 0 { "Q" } else { "-" },
        if rights & Castling::BK as u8 != 0 { "k" } else { "-" },
        if rights & Castling::BQ as u8 != 0 { "q" } else { "-" },
    )
}
