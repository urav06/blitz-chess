//! Chess game state and move generation.

use crate::board::{Board, Color, Square};

// ============================================================================
// Type Definitions
// ============================================================================

#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum CastlingSide { Kingside = 0, Queenside = 1 }

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct CastlingRights(u8);

pub struct State {
    pub board: Board,
    pub to_move: Color,
    pub castling_rights: CastlingRights,
    pub en_passant: Option<Square>,
    pub halfmove_clock: u8,
    pub fullmove_number: u16,
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
}

// ============================================================================
// State
// ============================================================================