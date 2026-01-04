//! Chess move representation. "move" is a reserved keyword in Rust, so we use "mv".

use crate::board::{PieceType, Square};
use crate::castling::CastlingSide;

// ============================================================================
// Type Definitions
// ============================================================================

#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MoveType {
    Normal = 0,
    Promotion = 1,
    EnPassant = 2,
    Castling = 3,
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Move(u16);

// ============================================================================
// MoveType
// ============================================================================

impl MoveType {
    pub const fn from_u8(value: u8) -> Self {
        match value {
            0 => MoveType::Normal,
            1 => MoveType::Promotion,
            2 => MoveType::EnPassant,
            3 => MoveType::Castling,
            _ => unreachable!(),
        }
    }
}

// ============================================================================
// Move
// ============================================================================

impl Move {
    // --- Bit encoding scheme --- //
    const SOURCE_MASK: u16 = 0b0000_0000_0011_1111;
    const TARGET_MASK: u16 = 0b0000_1111_1100_0000;
    const PROMO_MASK : u16 = 0b0011_0000_0000_0000;
    const TYPE_MASK  : u16 = 0b1100_0000_0000_0000;

    // --- Construction --- //
    pub const fn new(source: Square, target: Square) -> Self {
        Self( (source.value() as u16) | ((target.value() as u16) << 6) )
    }

    pub const fn promotion(source: Square, target: Square, promoted_to: PieceType) -> Self {
        Self(
            (source.value() as u16)
            | ((target.value() as u16) << 6)
            | ((promoted_to as u16) << 12)
            | ((MoveType::Promotion as u16) << 14)
        )
    }

    pub const fn en_passant(source: Square, target: Square) -> Self {
        Self(
            (source.value() as u16)
            | ((target.value() as u16) << 6)
            | ((MoveType::EnPassant as u16) << 14)
        )
    }

    pub const fn castling(source: Square, target: Square) -> Self {
        Self(
            (source.value() as u16)
            | ((target.value() as u16) << 6)
            | ((MoveType::Castling as u16) << 14)
        )
    }

    // --- Extraction --- //
    pub const fn source(self) -> Square {
        Square::from_index((self.0 & Self::SOURCE_MASK) as usize)
    }

    pub const fn target(self) -> Square {
        Square::from_index(((self.0 >> 6) & 0b111111) as usize)
    }

    pub const fn move_type(self) -> MoveType {
        MoveType::from_u8(((self.0 >> 14) & 0b11) as u8)
    }

    pub const fn promotion_piece(self) -> Option<PieceType> {
        match self.move_type() {
            MoveType::Promotion => Some(PieceType::from_u8(((self.0 >> 12) & 0b11) as u8)),
            _ => None
        }
    }

    // --- Derived (for special move types) --- //
    pub const fn castling_side(self) -> CastlingSide {
        if self.target().file() > self.source().file() { CastlingSide::Kingside } else { CastlingSide::Queenside }
    }

    pub const fn castling_rook_squares(self) -> (Square, Square) {
        let side = self.castling_side();
        let rank = self.source().rank();
        (
            Square::from_coords(rank, side.rook_source_file()),
            Square::from_coords(rank, side.rook_target_file()),
        )
    }

    pub const fn en_passant_capture(self) -> Square {
        Square::from_coords(self.source().rank(), self.target().file())
    }
}
