//! Chess move representation.

use crate::board::{PieceType, Square};

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
        Self( (source as u16) | ((target as u16) << 6) )
    }

    pub const fn promotion(source: Square, target: Square, promoted_to: PieceType) -> Self {
        Self(
            (source as u16)
            | ((target as u16) << 6)
            | ((promoted_to as u16) << 12)
            | ((MoveType::Promotion as u16) << 14)
        )
    }

    pub const fn en_passant(source: Square, target: Square) -> Self {
        Self(
            (source as u16)
            | ((target as u16) << 6)
            | ((MoveType::EnPassant as u16) << 14)
        )
    }

    pub const fn castling(source: Square, target: Square) -> Self {
        Self(
            (source as u16)
            | ((target as u16) << 6)
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
}
