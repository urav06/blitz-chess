//! Chess board representation and core data structures.

use std::fmt::{Display, Formatter, Result as FmtResult};
use std::mem::replace;
use std::ops::{Add, Index, IndexMut, Not};

use crate::display::{render_board, render_piece, render_square};

// ============================================================================
// Type Definitions
// ============================================================================

#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum PieceType {
    Knight = 0b000,
    Bishop = 0b001,
    Rook   = 0b010,
    Queen  = 0b011,
    Pawn   = 0b100,
    King   = 0b101,
}

#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Color { White = 0, Black = 1 }

#[repr(i8)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Lateral {
    Left        = -1,
    Straight    = 0,
    Right       = 1,
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Piece(u8);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Square(u8);

#[derive(Clone)]
pub struct Board { squares: [u8; 64] }

// ============================================================================
// Color
// ============================================================================

// --- Traits --- //
impl Not for Color {
    type Output = Self;
    fn not(self) -> Self {
        match self { Color::White => Color::Black, Color::Black => Color::White }
    }
}

// ============================================================================
// Square
// ============================================================================

impl Square {

    // --- Construction --- //
    pub const fn from_coords(rank: u8, file: u8) -> Self { Square((rank << 3) | file) }
    pub const fn from_index(index: usize) -> Self { Square(index as u8) }

    // --- Extraction --- //
    pub const fn value(self) -> u8 { self.0 }
    pub const fn rank(self) -> u8 { self.0 >> 3 }
    pub const fn file(self) -> u8 { self.0 & 0b111 }
    pub const fn index(self) -> usize { self.0 as usize }

    // --- Validation --- //
    pub const fn in_bounds(rank: i8, file: i8) -> bool { rank >= 0 && rank < 8 && file >= 0 && file < 8 }

    // --- Geometry --- //
    pub const fn offset(self, dr: i8, df: i8) -> Option<Self> {
        let (r, f) = (self.rank() as i8 + dr, self.file() as i8 + df);
        if Self::in_bounds(r, f) { Some(Self::from_coords(r as u8, f as u8)) } else { None }
    }

    pub const fn forward(self, color: Color, steps: i8, lateral: Lateral) -> Option<Self> {
        let (dr, df) = match color {
            Color::White => (steps, lateral as i8),
            Color::Black => (-steps, -(lateral as i8)),
        };
        self.offset(dr, df)
    }
}

// --- Traits --- //
impl From<Square> for usize {
    fn from(square: Square) -> usize { square.index() }
}

impl From<usize> for Square {
    fn from(index: usize) -> Self { Square::from_index(index) }
}

impl From<(u8, u8)> for Square {
    fn from((rank, file): (u8, u8)) -> Self { Square::from_coords(rank, file) }
}

impl From<Square> for (u8, u8) {
    fn from(square: Square) -> (u8, u8) { (square.rank(), square.file()) }
}

impl Display for Square {
    fn fmt(&self, f: &mut Formatter) -> FmtResult { render_square(self, f) }
}

impl Add<(i8, i8)> for Square {
    type Output = Option<Square>;
    fn add(self, (dr, df): (i8, i8)) -> Option<Square> {
        self.offset(dr, df)
    }
}

// ============================================================================
// PieceType
// ============================================================================

impl PieceType {

    /// Pieces a pawn can promote to. These are exactly the pieces where MSB is 0.
    pub const PROMOTABLE: [PieceType; 4] = [
        PieceType::Knight,
        PieceType::Bishop,
        PieceType::Rook,
        PieceType::Queen,
    ];

    pub(crate) const fn from_u8(value: u8) -> Self {
        match value {
            0b000 => Self::Knight,
            0b001 => Self::Bishop,
            0b010 => Self::Rook,
            0b011 => Self::Queen,
            0b100 => Self::Pawn,
            0b101 => Self::King,
            _ => unreachable!(),
        }
    }
}

// ============================================================================
// Piece
// ============================================================================

impl Piece {
    // --- Bit encoding scheme --- //
    const OCCUPIED_BIT  : u8 = 0b1000_0000; // Bit 7
    const MOVED_BIT     : u8 = 0b0100_0000; // Bit 6
    const COLOR_BIT     : u8 = 0b0000_1000; // Bit 3
    const PIECE_MASK    : u8 = 0b0000_0111; // Bits 2-0

    // --- Construction --- //
    pub const fn new(piece_type: PieceType, color: Color) -> Self {
        Piece(Self::OCCUPIED_BIT | ((color as u8) << 3) | (piece_type as u8))
    }

    pub const fn from_value(value: u8) -> Option<Self> {
        if Self::is_empty_value(value) { None } else { Some(Piece(value)) }
    }

    // --- Extraction --- //
    pub const fn value(self) -> u8 { self.0 }
    pub const fn has_moved(self) -> bool { self.0 & Self::MOVED_BIT != 0 }
    pub const fn piece_type(self) -> PieceType { PieceType::from_u8(self.0 & Self::PIECE_MASK) }

    pub const fn color(self) -> Color {
        if self.0 & Self::COLOR_BIT != 0 { Color::Black } else { Color::White }
    }

    // --- Modifications --- //
    pub const fn with_moved(self) -> Self { Piece(self.0 | Self::MOVED_BIT) }

    // --- Utilities --- //
    pub const fn is_empty_value(value: u8) -> bool { value & Self::OCCUPIED_BIT == 0 }
}

// --- Traits --- //
impl From<Piece> for u8 {
    fn from(piece: Piece) -> u8 { piece.value() }
}

impl TryFrom<u8> for Piece {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, ()> { Piece::from_value(value).ok_or(()) }
}

impl Display for Piece {
    fn fmt(&self, f: &mut Formatter) -> FmtResult { render_piece(self, f) }
}

// ============================================================================
// Board
// ============================================================================

impl Board {
    // --- Construction --- //
    pub const fn new() -> Self { Board { squares: [0; 64] } }

    // --- Queries --- //
    pub fn piece_at(&self, s: impl Into<Square>) -> Option<Piece> { Piece::from_value(self[s]) }
    pub fn is_empty(&self, s: impl Into<Square>) -> bool { Piece::is_empty_value(self[s]) }

    pub fn pieces(&self) -> impl Iterator<Item = (Square, Piece)> + '_ {
        (0..64).map(Square::from_index).filter_map(|sq| self.piece_at(sq).map(|p| (sq, p)))
    }

    // --- Mutations --- //
    pub fn set_piece(&mut self, p: Piece, s: impl Into<Square>) -> Option<Piece> {
        Piece::from_value(replace(&mut self[s], p.into()))
    }

    pub fn remove_piece(&mut self, s: impl Into<Square>) -> Option<Piece> {
        Piece::from_value(replace(&mut self[s], 0))
    }

    pub fn move_piece(&mut self, from: impl Into<Square>, to: impl Into<Square>) -> Option<Piece> {
        self.remove_piece(from).and_then(|piece| self.set_piece(piece.with_moved(), to))
    }
}

// --- Traits --- //
impl<T: Into<Square>> Index<T> for Board {
    type Output = u8;
    fn index(&self, s: T) -> &u8 { &self.squares[s.into().index()] }
}

impl<T: Into<Square>> IndexMut<T> for Board {
    fn index_mut(&mut self, s: T) -> &mut u8 { &mut self.squares[s.into().index()] }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter) -> FmtResult { render_board(self, f) }
}
