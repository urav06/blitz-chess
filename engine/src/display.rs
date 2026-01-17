//! Display formatting for chess types.

use std::fmt::{Formatter, Result};

use crate::board::{Board, Color, Piece, PieceType, Square};

// ============================================================================
// Unicode Piece Characters
// ============================================================================

const WHITE_KING    : char = '♔';
const WHITE_QUEEN   : char = '♕';
const WHITE_ROOK    : char = '♖';
const WHITE_BISHOP  : char = '♗';
const WHITE_KNIGHT  : char = '♘';
const WHITE_PAWN    : char = '♙';

const BLACK_KING    : char = '♚';
const BLACK_QUEEN   : char = '♛';
const BLACK_ROOK    : char = '♜';
const BLACK_BISHOP  : char = '♝';
const BLACK_KNIGHT  : char = '♞';
const BLACK_PAWN    : char = '♟';

const EMPTY         : char = '·';

// ============================================================================
// Public Rendering Functions
// ============================================================================

pub fn render_square(square: &Square, f: &mut Formatter) -> Result {
    write!(f, "{}{}", (b'a' + square.file()) as char, square.rank() + 1)
}

pub fn render_piece(piece: &Piece, f: &mut Formatter) -> Result {
    write!(f, "{}", piece_char(piece))
}

pub fn render_board(board: &Board, f: &mut Formatter) -> Result {
    const COORDS: &str = "  a b c d e f g h";
    // Top coordinate row
    writeln!(f, "{}", COORDS)?;
    // Render ranks from 8 down to 1 (White's perspective)
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

    // Bottom coordinate row
    write!(f, "{}", COORDS)
}

// ============================================================================
// Private Helpers
// ============================================================================

fn piece_char(piece: &Piece) -> char {
    match (piece.color(), piece.piece_type()) {
        (Color::White, PieceType::King)     => WHITE_KING,
        (Color::White, PieceType::Queen)    => WHITE_QUEEN,
        (Color::White, PieceType::Rook)     => WHITE_ROOK,
        (Color::White, PieceType::Bishop)   => WHITE_BISHOP,
        (Color::White, PieceType::Knight)   => WHITE_KNIGHT,
        (Color::White, PieceType::Pawn)     => WHITE_PAWN,
        (Color::Black, PieceType::King)     => BLACK_KING,
        (Color::Black, PieceType::Queen)    => BLACK_QUEEN,
        (Color::Black, PieceType::Rook)     => BLACK_ROOK,
        (Color::Black, PieceType::Bishop)   => BLACK_BISHOP,
        (Color::Black, PieceType::Knight)   => BLACK_KNIGHT,
        (Color::Black, PieceType::Pawn)     => BLACK_PAWN,
    }
}
