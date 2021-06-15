#![feature(stmt_expr_attributes)]

pub mod bitboard;
pub mod constants;
pub mod enums;
pub mod game;
mod macros;
mod cli;
pub mod pieces;
pub mod position;

#[cfg(test)]
mod test;
