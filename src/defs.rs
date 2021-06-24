use std::fmt;

#[rustfmt::skip]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Square {
  A8, B8, C8, D8, E8, F8, G8, H8,
  A7, B7, C7, D7, E7, F7, G7, H7,
  A6, B6, C6, D6, E6, F6, G6, H6,
  A5, B5, C5, D5, E5, F5, G5, H5,
  A4, B4, C4, D4, E4, F4, G4, H4,
  A3, B3, C3, D3, E3, F3, G3, H3,
  A2, B2, C2, D2, E2, F2, G2, H2,
  A1, B1, C1, D1, E1, F1, G1, H1,
  NoSquare
}

#[derive(Debug, Copy, Clone)]
pub enum Piece {
    WhiteKing,
    WhiteQueen,
    WhiteRook,
    WhiteBishop,
    WhiteKnight,
    WhitePawn,
    BlackKing,
    BlackQueen,
    BlackRook,
    BlackBishop,
    BlackKnight,
    BlackPawn,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Side {
    White,
    Black,
}

impl fmt::Display for Side {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<&str> for Side {
    fn from(s: &str) -> Side {
        match s {
            "b" => Side::Black,
            _ => Side::White,
        }
    }
}

pub enum Axis {
    Rank,
    File,
}

impl From<Piece> for Side {
    fn from(piece: Piece) -> Self {
        match piece {
            Piece::WhiteKing
            | Piece::WhiteQueen
            | Piece::WhiteRook
            | Piece::WhiteBishop
            | Piece::WhiteKnight
            | Piece::WhitePawn => Self::White,
            _ => Self::Black,
        }
    }
}

// Castling rights are represented by a 4 bit unsigned integer
// Each bit represents a single castling right
//
// https://www.chessprogramming.org/Castling_Rights
pub type CastlingRights = u8;
pub struct Castling;
impl Castling {
    pub const WK: u8 = 1;
    pub const WQ: u8 = 2;
    pub const BK: u8 = 4;
    pub const BQ: u8 = 8;
}

// This struct holds array indexes of the position bitboards
// It is not an enum for convinience of not casting the values to usize
pub struct Bitboard;
impl Bitboard {
    pub const WHITE_KING: usize = 0;
    pub const WHITE_QUEENS: usize = 1;
    pub const WHITE_ROOKS: usize = 2;
    pub const WHITE_BISHOPS: usize = 3;
    pub const WHITE_KNIGHTS: usize = 4;
    pub const WHITE_PAWNS: usize = 5;
    pub const BLACK_KING: usize = 6;
    pub const BLACK_QUEENS: usize = 7;
    pub const BLACK_ROOKS: usize = 8;
    pub const BLACK_BISHOPS: usize = 9;
    pub const BLACK_KNIGHTS: usize = 10;
    pub const BLACK_PAWNS: usize = 11;
    pub const WHITE_PIECES: usize = 12;
    pub const BLACK_PIECES: usize = 13;
    pub const ALL_PIECES: usize = 14;
}
