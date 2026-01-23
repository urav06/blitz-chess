//! Castling types and logic.

use crate::board::{Color, Square};

// ============================================================================
// Type Definitions
// ============================================================================

#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum CastlingSide { Kingside = 0, Queenside = 1 }

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct CastlingRights(u8);

// ============================================================================
// Castling Side
// ============================================================================

impl CastlingSide {

    // --- Constant Files --- //
    pub const KING_FILE: u8 = 4;

    const KING_TARGETS:  [u8; 2] = [6, 2];
    const ROOK_SOURCES:  [u8; 2] = [7, 0];
    const ROOK_TARGETS:  [u8; 2] = [5, 3];
    const CORRIDORS:     [&'static [u8]; 2] = [&[5, 6], &[1, 2, 3]];
    const KING_PATHS:    [&'static [u8]; 2] = [&[5, 6], &[3, 2]];

    // --- File accessors --- //
    pub const fn king_target_file(self) -> u8 { Self::KING_TARGETS[self as usize] }
    pub const fn rook_source_file(self) -> u8 { Self::ROOK_SOURCES[self as usize] }
    pub const fn rook_target_file(self) -> u8 { Self::ROOK_TARGETS[self as usize] }
    pub const fn corridor_files(self) -> &'static [u8] { Self::CORRIDORS[self as usize] }
    pub const fn king_path_files(self) -> &'static [u8] { Self::KING_PATHS[self as usize] }

    /// Returns the castling side if this file is a rook home file.
    pub const fn from_rook_file(file: u8) -> Option<Self> {
        if file == Self::ROOK_SOURCES[Self::Kingside as usize] { return Some(Self::Kingside); }
        if file == Self::ROOK_SOURCES[Self::Queenside as usize] { return Some(Self::Queenside); }
        None
    }

    /// Returns the castling side if this square is a rook's home square for the given color.
    pub fn from_rook_square(square: Square, color: Color) -> Option<Self> {
        if square.rank() != color.home_rank() { return None; }
        Self::from_rook_file(square.file())
    }
}

// ============================================================================
// Castling Rights
// ============================================================================

impl CastlingRights {

    const fn bit_position(c: Color, s: CastlingSide) -> u8 { (c as u8) * 2 + (s as u8) }

    // --- Construction --- //
    pub const fn none() -> Self { CastlingRights(0) }
    pub const fn all() -> Self { CastlingRights(0b1111) }

    // --- Query --- //
    pub const fn has(self, color: Color, side: CastlingSide) -> bool {
        let bit = 1 << Self::bit_position(color, side);
        (self.0 & bit) != 0
    }

    pub const fn is_empty(self) -> bool { self.0 == 0 }

    pub const fn any(self, color: Color) -> bool {
        self.has(color, CastlingSide::Kingside) || self.has(color, CastlingSide::Queenside)
    }

    // --- Modifications --- //
    pub const fn gain(self, color: Color, side: CastlingSide) -> Self {
        let bit = 1 << Self::bit_position(color, side);
        CastlingRights(self.0 | bit)
    }

    pub const fn lose(self, color: Color, side: CastlingSide) -> Self {
        let bit = 1 << Self::bit_position(color, side);
        CastlingRights(self.0 & !bit)
    }

    pub const fn lose_all(self, color: Color) -> Self {
        self.lose(color, CastlingSide::Kingside)
            .lose(color, CastlingSide::Queenside)
    }

    /// Lose rights if the given square is a rook's home square for this color.
    pub fn lose_for_rook_at(self, square: Square, color: Color) -> Self {
        match CastlingSide::from_rook_square(square, color) {
            Some(side) => self.lose(color, side),
            None => self,
        }
    }
}
