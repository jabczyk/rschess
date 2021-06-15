pub const PIECE_CHARS: [char; 12] = ['K', 'Q', 'R', 'B', 'N', 'P', 'k', 'q', 'r', 'b', 'n', 'p'];

#[rustfmt::skip]
pub const HUMAN_SQUARES: [&str; 64] = [
  "a8", "b8", "c8", "d8", "e8", "f8", "g8", "h8",
  "a7", "b7", "c7", "d7", "e7", "f7", "g7", "h7",
  "a6", "b6", "c6", "d6", "e6", "f6", "g6", "h6",
  "a5", "b5", "c5", "d5", "e5", "f5", "g5", "h5",
  "a4", "b4", "c4", "d4", "e4", "f4", "g4", "h4",
  "a3", "b3", "c3", "d3", "e3", "f3", "g3", "h3",
  "a2", "b2", "c2", "d2", "e2", "f2", "g2", "h2",
  "a1", "b1", "c1", "d1", "e1", "f1", "g1", "h1",
];

pub const A_FILE: u64 = 72340172838076673;
pub const B_FILE: u64 = 144680345676153346;
pub const AB_FILE: u64 = A_FILE | B_FILE;
pub const G_FILE: u64 = 4629771061636907072;
pub const H_FILE: u64 = 9259542123273814144;
pub const GH_FILE: u64 = G_FILE | H_FILE;
