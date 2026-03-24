//! Chess board representation and core data structures.

use std::fmt::{Display, Formatter, Result as FmtResult};
use std::num::NonZeroU8;
use std::ops::{Index, IndexMut, Not};

use crate::display::{render_board, render_piece, render_square};

// ============================================================================
// Type Definitions
// ============================================================================

#[rustfmt::skip]
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

#[rustfmt::skip]
#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Color { White = 0, Black = 1 }

#[rustfmt::skip]
#[repr(i8)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Lateral {
    Left     = -1,
    Straight =  0,
    Right    =  1,
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Piece(NonZeroU8); // niche optimization

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Square(u8);

#[derive(Clone)]
pub struct Board {
    squares: [Option<Piece>; 64],
}

// ============================================================================
// Color
// ============================================================================

#[rustfmt::skip]
impl Color {
    pub const fn home_rank(self) -> u8 {
        match self { Self::White => 0, Self::Black => 7 }
    }
    pub const fn pawn_rank(self) -> u8 {
        match self { Self::White => 1, Self::Black => 6 }
    }
    pub const fn promotion_rank(self) -> u8 {
        match self { Self::White => 7, Self::Black => 0 }
    }
}

#[rustfmt::skip]
impl Not for Color {
    type Output = Self;
    fn not(self) -> Self {
        match self { Self::White => Self::Black, Self::Black => Self::White }
    }
}

// ============================================================================
// Lateral
// ============================================================================

impl Lateral {
    pub const CAPTURES: [Self; 2] = [Self::Left, Self::Right];
}

// ============================================================================
// Square
// ============================================================================

impl Square {
    // --- Construction --- //
    pub const fn from_coords(rank: u8, file: u8) -> Self {
        Self((rank << 3) | file)
    }
    pub const fn from_index(index: usize) -> Self {
        Self(index as u8)
    }

    // --- Extraction --- //
    pub const fn value(self) -> u8 {
        self.0
    }
    pub const fn rank(self) -> u8 {
        self.0 >> 3
    }
    pub const fn file(self) -> u8 {
        self.0 & 0b111
    }
    pub const fn index(self) -> usize {
        self.0 as usize
    }

    // --- Validation --- //
    pub const fn in_bounds(rank: i8, file: i8) -> bool {
        rank >= 0 && rank < 8 && file >= 0 && file < 8
    }

    // --- Geometry --- //
    pub const fn offset(self, dr: i8, df: i8) -> Option<Self> {
        let (r, f) = (self.rank() as i8 + dr, self.file() as i8 + df);
        if Self::in_bounds(r, f) {
            Some(Self::from_coords(r as u8, f as u8))
        } else {
            None
        }
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
    fn from(square: Square) -> usize {
        square.index()
    }
}

impl From<usize> for Square {
    fn from(index: usize) -> Self {
        Self::from_index(index)
    }
}

impl From<(u8, u8)> for Square {
    fn from((rank, file): (u8, u8)) -> Self {
        Self::from_coords(rank, file)
    }
}

impl From<Square> for (u8, u8) {
    fn from(square: Square) -> (u8, u8) {
        (square.rank(), square.file())
    }
}

impl Display for Square {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        render_square(self, f)
    }
}

// ============================================================================
// PieceType
// ============================================================================

impl PieceType {
    /// Pieces a pawn can promote to. These are exactly the pieces where MSB is 0.
    pub const PROMOTABLE: [Self; 4] = [Self::Knight, Self::Bishop, Self::Rook, Self::Queen];

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
    const OCCUPIED_BIT: u8 = 0b1000_0000; // Bit 7
    const COLOR_BIT: u8 = 0b0000_1000; // Bit 3
    const PIECE_MASK: u8 = 0b0000_0111; // Bits 2-0

    // --- Construction --- //
    pub const fn new(piece_type: PieceType, color: Color) -> Self {
        let bits = Self::OCCUPIED_BIT | ((color as u8) << 3) | (piece_type as u8);
        // SAFETY: OCCUPIED_BIT guarantees non-zero value
        unsafe { Self(NonZeroU8::new_unchecked(bits)) }
    }

    // --- Extraction --- //
    pub const fn piece_type(self) -> PieceType {
        PieceType::from_u8(self.0.get() & Self::PIECE_MASK)
    }

    pub const fn color(self) -> Color {
        if self.0.get() & Self::COLOR_BIT != 0 {
            Color::Black
        } else {
            Color::White
        }
    }

    // --- Type Checks --- //
    pub const fn is_pawn(self) -> bool {
        matches!(self.piece_type(), PieceType::Pawn)
    }
    pub const fn is_king(self) -> bool {
        matches!(self.piece_type(), PieceType::King)
    }
}

// --- Traits --- //
impl Display for Piece {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        render_piece(self, f)
    }
}

// ============================================================================
// Board
// ============================================================================

impl Board {
    // --- Construction --- //
    pub const fn new() -> Self {
        Self {
            squares: [None; 64],
        }
    }

    // --- Queries --- //
    pub fn find<'a, F>(&'a self, f: F) -> impl Iterator<Item = (Square, Piece)> + 'a
    where
        F: Fn(Piece) -> bool + 'a,
    {
        self.squares
            .iter()
            .enumerate()
            .filter_map(move |(i, slot)| slot.filter(|&p| f(p)).map(|p| (Square::from_index(i), p)))
    }

    pub fn pieces(&self) -> impl Iterator<Item = (Square, Piece)> + '_ {
        self.find(|_| true)
    }

    pub fn is_landable(&self, sq: Square, by: Color) -> bool {
        self[sq].is_none_or(|p| p.color() != by)
    }

    // --- Mutations --- //
    pub fn move_piece(&mut self, from: impl Into<Square>, to: impl Into<Square>) -> Option<Piece> {
        self[from].lift().and_then(|piece| self[to].place(piece))
    }
}

// --- Traits --- //
pub trait SlotExt {
    fn place(&mut self, piece: Piece) -> Option<Piece>;
    fn lift(&mut self) -> Option<Piece>;
}

impl SlotExt for Option<Piece> {
    fn place(&mut self, piece: Piece) -> Option<Piece> {
        self.replace(piece)
    }
    fn lift(&mut self) -> Option<Piece> {
        self.take()
    }
}

impl<T: Into<Square>> Index<T> for Board {
    type Output = Option<Piece>;

    fn index(&self, s: T) -> &Option<Piece> {
        &self.squares[s.into().index()]
    }
}

impl<T: Into<Square>> IndexMut<T> for Board {
    fn index_mut(&mut self, s: T) -> &mut Option<Piece> {
        &mut self.squares[s.into().index()]
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        render_board(self, f)
    }
}
