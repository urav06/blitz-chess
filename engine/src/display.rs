//! Display formatting for chess types.

use std::fmt::{Formatter, Result};

use crate::board::{Board, Piece, Square};

// ============================================================================
// Piece Characters
// ============================================================================

#[rustfmt::skip]
const PIECE_CHARS: [[char; 6]; 2] = [
    ['♘', '♗', '♖', '♕', '♙', '♔'], // White: Knight Bishop Rook Queen Pawn King
    ['♞', '♝', '♜', '♛', '♟', '♚'], // Black
];

const EMPTY: char = '·';

// ============================================================================
// Rendering
// ============================================================================

pub fn render_square(square: &Square, f: &mut Formatter) -> Result {
    write!(f, "{}{}", (b'a' + square.file()) as char, square.rank() + 1)
}

pub fn render_piece(piece: &Piece, f: &mut Formatter) -> Result {
    let ch = PIECE_CHARS[piece.color() as usize][piece.piece_type() as usize];
    write!(f, "{ch}")
}

pub fn render_board(board: &Board, f: &mut Formatter) -> Result {
    const COORDS: &str = "  a b c d e f g h";
    writeln!(f, "{}", COORDS)?;
    for rank in (0..8).rev() {
        write!(f, "{} ", rank + 1)?;
        for file in 0..8 {
            let square = Square::from_coords(rank, file);
            match board[square] {
                Some(piece) => write!(f, "{} ", piece)?,
                None => write!(f, "{} ", EMPTY)?,
            }
        }
        writeln!(f, "{}", rank + 1)?;
    }
    write!(f, "{}", COORDS)
}
