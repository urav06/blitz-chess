//! Move generation.

use crate::board::{Board, Color, PieceType, Square};
use crate::mv::Move;
use crate::state::State;

// ============================================================================
// Type Definitions
// ============================================================================

pub struct MoveGenerator<'a> {
    state: &'a State,
}

// ============================================================================
// MoveGenerator — Public Interface
// ============================================================================

impl<'a> MoveGenerator<'a> {

    /// Create a new move generator for the given state.
    pub fn new(state: &'a State) -> Self {
        MoveGenerator { state }
    }

    /// Generate all legal moves for the current side to move.
    pub fn all(self) -> impl Iterator<Item = Move> + 'a {
        gen move {
            for mv in pseudo_legal_moves(self.state) {
                if is_legal(self.state, mv) {
                    yield mv;
                }
            }
        }
    }

    /// Generate legal moves from a specific square.
    pub fn from(self, sq: Square) -> impl Iterator<Item = Move> + 'a {
        gen move {
            for mv in pseudo_legal_moves(self.state) {
                if mv.source() == sq && is_legal(self.state, mv) {
                    yield mv;
                }
            }
        }
    }
}

// ============================================================================
// Pseudo-Legal Move Generation
// ============================================================================

fn pseudo_legal_moves(state: &State) -> impl Iterator<Item = Move> + '_ {
    gen move {
        for (sq, piece) in state.board.pieces() {
            if piece.color() != state.to_move {
                continue;
            }

            match piece.piece_type() {
                PieceType::Pawn   => { for mv in pawn_moves(state, sq)   { yield mv; } }
                PieceType::Knight => { for mv in knight_moves(state, sq) { yield mv; } }
                PieceType::Bishop => { for mv in bishop_moves(state, sq) { yield mv; } }
                PieceType::Rook   => { for mv in rook_moves(state, sq)   { yield mv; } }
                PieceType::Queen  => { for mv in queen_moves(state, sq)  { yield mv; } }
                PieceType::King   => { for mv in king_moves(state, sq)   { yield mv; } }
            }
        }
    }
}

// ============================================================================
// Piece-Specific Move Generation
// ============================================================================

// --- Knight --- //

const KNIGHT_OFFSETS: [(i8, i8); 8] = [
    (-2, -1), (-2, 1), (-1, -2), (-1, 2),
    ( 1, -2), ( 1, 2), ( 2, -1), ( 2, 1),
];

fn knight_moves(state: &State, from: Square) -> impl Iterator<Item = Move> + '_ {
    gen move {
        let color = state.to_move;
        for (dr, df) in KNIGHT_OFFSETS {
            if let Some(to) = from.offset(dr, df) {
                match state.board[to] {
                    None => yield Move::new(from, to),
                    Some(target) if target.color() != color => yield Move::new(from, to),
                    Some(_) => {}  // blocked by own piece
                }
            }
        }
    }
}

// --- Pawn --- //

fn pawn_moves(_state: &State, _from: Square) -> impl Iterator<Item = Move> + '_ {
    gen move {
        // TODO: implement pawn moves
    }
}

// --- Bishop --- //

fn bishop_moves(_state: &State, _from: Square) -> impl Iterator<Item = Move> + '_ {
    gen move {
        // TODO: implement bishop moves (diagonal sliding)
    }
}

// --- Rook --- //

fn rook_moves(_state: &State, _from: Square) -> impl Iterator<Item = Move> + '_ {
    gen move {
        // TODO: implement rook moves (orthogonal sliding)
    }
}

// --- Queen --- //

fn queen_moves(_state: &State, _from: Square) -> impl Iterator<Item = Move> + '_ {
    gen move {
        // TODO: implement queen moves (bishop + rook)
    }
}

// --- King --- //

fn king_moves(_state: &State, _from: Square) -> impl Iterator<Item = Move> + '_ {
    gen move {
        // TODO: implement king moves (1-square in any direction + castling)
    }
}

// ============================================================================
// Legality Checking
// ============================================================================

/// Check if a pseudo-legal move is actually legal (doesn't leave king in check).
fn is_legal(state: &State, mv: Move) -> bool {
    let new_state = state.clone().apply_move(mv);
    let king_sq = find_king(&new_state.board, state.to_move);
    !is_square_attacked(&new_state.board, king_sq, !state.to_move)
}

/// Find the king of a given color on the board.
fn find_king(board: &Board, color: Color) -> Square {
    board.pieces()
        .find(|(_, p)| p.piece_type() == PieceType::King && p.color() == color)
        .map(|(sq, _)| sq)
        .expect("king must exist")
}

// ============================================================================
// Attack Detection
// ============================================================================

/// Check if a square is attacked by pieces of a given color.
pub fn is_square_attacked(board: &Board, square: Square, by: Color) -> bool {
    // Check knight attacks
    for (dr, df) in KNIGHT_OFFSETS {
        if let Some(sq) = square.offset(dr, df) {
            if let Some(piece) = board[sq] {
                if piece.color() == by && piece.piece_type() == PieceType::Knight {
                    return true;
                }
            }
        }
    }

    // TODO: Check sliding piece attacks (bishop, rook, queen)
    // TODO: Check pawn attacks
    // TODO: Check king attacks

    false
}

/// Check if the current side to move is in check.
pub fn is_in_check(state: &State) -> bool {
    let king_sq = find_king(&state.board, state.to_move);
    is_square_attacked(&state.board, king_sq, !state.to_move)
}
