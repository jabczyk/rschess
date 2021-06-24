#![feature(stmt_expr_attributes)]

pub mod bitboard;
mod cli;
pub mod constants;
pub mod defs;
mod fen;
pub mod game;
mod macros;
pub mod pieces;
pub mod position;

#[cfg(test)]
mod test;
