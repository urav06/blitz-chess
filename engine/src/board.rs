//! Chess board representation and core data structures.

use std::fmt::{Display, Formatter, Result as FmtResult};
use std::ops::{Index, IndexMut};

use crate::display::{render_board, render_piece, render_square};

// ============================================================================
// Type Definitions
// ============================================================================

#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum PieceType {
    Pawn    = 1,
    Knight  = 2,
    Bishop  = 3,
    Rook    = 4,
    Queen   = 5,
    King    = 6,
}

#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Color { White = 0, Black = 1 }

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Piece(u8);

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Square(u8);

#[derive(Clone)]
pub struct Board { squares: [u8; 64] }

// ============================================================================
// Square
// ============================================================================

impl Square {
    // --- Construction --- //
    pub const fn from_coords(rank: u8, file: u8) -> Self { Square((rank << 3) | file) }
    pub const fn from_index(index: usize) -> Self { Square(index as u8) }

    // --- Extraction --- //
    pub const fn rank(self) -> u8 { self.0 >> 3 }
    pub const fn file(self) -> u8 { self.0 & 0b111 }
    pub const fn index(self) -> usize { self.0 as usize }
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

    pub const fn piece_type(self) -> PieceType {
        match self.0 & Self::PIECE_MASK {
            1 => PieceType::Pawn,
            2 => PieceType::Knight,
            3 => PieceType::Bishop,
            4 => PieceType::Rook,
            5 => PieceType::Queen,
            6 => PieceType::King,
            _ => unreachable!(),
        }
    }

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

    // --- Modifications --- //
    pub fn with_piece(mut self, p: Piece, s: impl Into<Square>) -> Self {
        self[s] = p.into();
        self
    }
    pub fn without_piece(mut self, s: impl Into<Square>) -> Self {
        self[s] = 0;
        self
    }

    pub fn with_move(mut self, from: impl Into<Square>, to: impl Into<Square>) -> Self {
        let from = from.into();
        let to = to.into();
        if let Some(piece) = self.piece_at(from) {
            self[from] = 0;
            self[to] = piece.with_moved().value();
        }
        self
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
