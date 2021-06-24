use crate::{
    bitboard::*,
    constants::HUMAN_SQUARES,
    defs::{Bitboard, Castling, CastlingRights, Side, Square},
    position::Position,
};

impl Position {
    // Creates a position from [FEN] string
    // TODO: Return `Err` on invalid FEN strings
    //
    // [FEN]: https://www.chessprogramming.org/Forsyth-Edwards_Notation
    pub fn from_fen(fen: &str) -> Self {
        let fen_parts: Vec<&str> = fen.split_whitespace().collect();
        assert!(fen_parts.len() == 4 || fen_parts.len() == 6);

        Self {
            bitboards: place_pieces(fen_parts[0]),
            en_passant_square: get_square_id(fen_parts[3]),
            castling_rights: get_castling_rights(fen_parts[2]),
            side_to_move: fen_parts[1].into(),
            fifty_move_count: fen_parts.get(4).unwrap_or(&"0").parse().unwrap(),
            halfmove_count: get_halfmoves(
                fen_parts.get(5).unwrap_or(&"1").parse().unwrap(),
                fen_parts[1].into(),
            ),
        }
    }
}

// Takes piece placement part of a FEN string, returns an array of bitboards
// representing the placement
fn place_pieces(placement: &str) -> [u64; 15] {
    let mut bitboards = [0u64; 15];
    let (mut rank, mut file) = (0u8, 0u8);

    for c in placement.chars() {
        let mut set_piece = |board_index: usize| {
            set_bit(&mut bitboards[board_index], sq(rank, file));
            file += 1;
        };

        match c {
            'K' => set_piece(Bitboard::WHITE_KING),
            'Q' => set_piece(Bitboard::WHITE_QUEENS),
            'R' => set_piece(Bitboard::WHITE_ROOKS),
            'B' => set_piece(Bitboard::WHITE_BISHOPS),
            'N' => set_piece(Bitboard::WHITE_KNIGHTS),
            'P' => set_piece(Bitboard::WHITE_PAWNS),
            'k' => set_piece(Bitboard::BLACK_KING),
            'q' => set_piece(Bitboard::BLACK_QUEENS),
            'r' => set_piece(Bitboard::BLACK_ROOKS),
            'b' => set_piece(Bitboard::BLACK_BISHOPS),
            'n' => set_piece(Bitboard::BLACK_KNIGHTS),
            'p' => set_piece(Bitboard::BLACK_PAWNS),
            '1'..='8' => file += c as u8 - 0x30,
            '/' => {
                rank += 1;
                file = 0;
            }
            _ => (),
        }
    }

    bitboards[Bitboard::WHITE_PIECES] = sum_bitboards(&bitboards[0..6]);
    bitboards[Bitboard::BLACK_PIECES] = sum_bitboards(&bitboards[6..12]);
    bitboards[Bitboard::ALL_PIECES] =
        bitboards[Bitboard::WHITE_PIECES] | bitboards[Bitboard::BLACK_PIECES];

    bitboards
}

// Parses FEN castling rights string (like "KQkq"), returns castling right integer
fn get_castling_rights(rights_str: &str) -> CastlingRights {
    let mut rights = 0;

    for c in rights_str.chars() {
        match c {
            'K' => rights |= Castling::WK,
            'Q' => rights |= Castling::WQ,
            'k' => rights |= Castling::BK,
            'q' => rights |= Castling::BQ,
            _ => break,
        }
    }

    rights
}

// Takes a human-readable square representation (like "e4")
// and returns the square ID (0-63) or "no square" (64)
fn get_square_id(square_str: &str) -> u8 {
    if square_str == "-" {
        return Square::NoSquare as u8;
    };

    let square_str = square_str.to_lowercase();

    HUMAN_SQUARES
        .iter()
        .position(|&square| square == square_str)
        .unwrap_or(Square::NoSquare as usize) as u8
}

// Returns total halfmove count in a game
fn get_halfmoves(full_moves: u16, side_to_move: Side) -> u16 {
    (full_moves * 2) + if side_to_move == Side::Black { 1 } else { 0 }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::defs::Piece::*;
    use crate::map;
    use Square::*;

    #[test]
    fn gets_halfmoves() {
        assert_eq!(get_halfmoves(0, Side::White), 0);
        assert_eq!(get_halfmoves(0, Side::Black), 1);
        assert_eq!(get_halfmoves(10, Side::White), 20);
        assert_eq!(get_halfmoves(10, Side::Black), 21);
    }

    #[test]
    fn gets_castling_rights() {
        assert_eq!(get_castling_rights("-"), 0);
        assert_eq!(
            get_castling_rights("KQkq"),
            Castling::WK | Castling::WQ | Castling::BK | Castling::BQ
        );
        assert_eq!(get_castling_rights("Qkq"), Castling::WQ | Castling::BK | Castling::BQ);
        assert_eq!(get_castling_rights("Kk"), Castling::WK | Castling::BK);
    }

    #[test]
    fn gets_square_id() {
        assert_eq!(get_square_id("-"), NoSquare as u8);
        assert_eq!(get_square_id("a1"), A1 as u8);
        assert_eq!(get_square_id("h8"), H8 as u8);
        assert_eq!(get_square_id("E4"), E4 as u8);
        assert_eq!(get_square_id("4E"), NoSquare as u8);
    }

    #[test]
    fn parses_empty_fen() {
        assert_eq!(
            Position::from_fen(crate::constants::EMPTY_FEN),
            Position {
                bitboards: [0; 15],
                en_passant_square: NoSquare as u8,
                castling_rights: 0,
                side_to_move: Side::White,
                fifty_move_count: 0,
                halfmove_count: 2
            }
        );
    }

    #[test]
    fn parses_starting_fen() {
        assert_eq!(
            Position::from_fen(crate::constants::STARTING_FEN),
            Position::from_position(map! {
                A1 => WhiteRook, H1 => WhiteRook, A8 => BlackRook, H8 => BlackRook,
                B1 => WhiteKnight, G1 => WhiteKnight, B8 => BlackKnight, G8 => BlackKnight,
                C1 => WhiteBishop, F1 => WhiteBishop, C8 => BlackBishop, F8 => BlackBishop,
                D1 => WhiteQueen, D8 => BlackQueen,
                E1 => WhiteKing, E8 => BlackKing,
                A2 => WhitePawn, B2 => WhitePawn, C2 => WhitePawn, D2 => WhitePawn,
                E2 => WhitePawn, F2 => WhitePawn, G2 => WhitePawn, H2 => WhitePawn,
                A7 => BlackPawn, B7 => BlackPawn, C7 => BlackPawn, D7 => BlackPawn,
                E7 => BlackPawn, F7 => BlackPawn, G7 => BlackPawn, H7 => BlackPawn
            })
        );
    }

    #[test]
    fn parses_random_fen() {
        let bitboards = Position::from_position(map! {
            E1 => WhiteKing, F7 => BlackKing,
            D2 => WhiteQueen, D6 => BlackQueen,
            E4 => WhiteKnight, C6 => BlackKnight,
            C1 => WhiteBishop, D7 => BlackBishop,
            H1 => WhiteRook, B8 => BlackRook,
            C4 => WhitePawn, B4 => BlackPawn
        })
        .bitboards;

        assert_eq!(
            Position::from_fen("1r6/3b1k2/2nq4/8/1pP1N3/8/3Q4/2B1K2R b K c3 1 27"),
            Position {
                bitboards,
                en_passant_square: get_square_id("c3"),
                castling_rights: Castling::WK,
                side_to_move: Side::Black,
                fifty_move_count: 1,
                halfmove_count: 55
            }
        );
    }

    #[test]
    #[should_panic]
    fn panics_on_invalid_length_fen() {
        Position::from_fen("one two three four five");
    }
}
