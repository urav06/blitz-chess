//! Move generation.

use crate::board::{Board, Color, Lateral, PieceType, Square};
use crate::castling::CastlingSide;
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
    pub(crate) fn new(state: &'a State) -> Self {
        Self { state }
    }

    pub fn all(self) -> impl Iterator<Item = Move> + 'a {
        gen move {
            for mv in pseudo_legal_moves(self.state) {
                if is_legal(self.state, mv) {
                    yield mv;
                }
            }
        }
    }

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

#[rustfmt::skip]
fn pseudo_legal_moves(state: &State) -> impl Iterator<Item = Move> + '_ {
    let color = state.to_move;
    gen move {
        for i in 0..64usize {
            let sq = Square::from_index(i);
            let Some(piece) = state.board[sq] else { continue };
            if piece.color() != color { continue; }
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
// Direction Constants
// ============================================================================

#[rustfmt::skip]
const KNIGHT_OFFSETS: [(i8, i8); 8] = [
    (-2, -1), (-2, 1), (-1, -2), (-1, 2),
    ( 1, -2), ( 1, 2), ( 2, -1), ( 2, 1),
];

#[rustfmt::skip]
const DIAGONALS: [(i8, i8); 4] = [(-1, -1), (-1, 1), (1, -1), (1, 1)];
#[rustfmt::skip]
const ORTHOGONALS: [(i8, i8); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
#[rustfmt::skip]
const ALL_DIRECTIONS: [(i8, i8); 8] = [
    (-1, -1), (-1, 0), (-1, 1),
    ( 0, -1),          ( 0, 1),
    ( 1, -1), ( 1, 0), ( 1, 1),
];

// ============================================================================
// Shared Movement Helpers
// ============================================================================

fn step_moves<'a>(
    board: &'a Board,
    from: Square,
    color: Color,
    offsets: &'a [(i8, i8)],
) -> impl Iterator<Item = Move> + 'a {
    gen move {
        for &(dr, df) in offsets {
            if let Some(to) = from.offset(dr, df)
                && board.is_landable(to, color)
            {
                yield Move::new(from, to);
            }
        }
    }
}

fn slide_moves<'a>(
    board: &'a Board,
    from: Square,
    color: Color,
    directions: &'a [(i8, i8)],
) -> impl Iterator<Item = Move> + 'a {
    gen move {
        for &(dr, df) in directions {
            let mut sq = from;
            while let Some(next) = sq.offset(dr, df) {
                match board[next] {
                    None => {
                        yield Move::new(from, next);
                        sq = next;
                    }
                    Some(p) if p.color() != color => {
                        yield Move::new(from, next);
                        break;
                    }
                    Some(_) => break,
                }
            }
        }
    }
}

fn pawn_move_or_promote(from: Square, to: Square, color: Color) -> impl Iterator<Item = Move> {
    gen move {
        if to.rank() == color.promotion_rank() {
            for pt in PieceType::PROMOTABLE {
                yield Move::promotion(from, to, pt);
            }
        } else {
            yield Move::new(from, to);
        }
    }
}

// ============================================================================
// Piece-Specific Move Generation
// ============================================================================

// --- Knight --- //

fn knight_moves(state: &State, from: Square) -> impl Iterator<Item = Move> + '_ {
    step_moves(&state.board, from, state.to_move, &KNIGHT_OFFSETS)
}

// --- Bishop --- //

fn bishop_moves(state: &State, from: Square) -> impl Iterator<Item = Move> + '_ {
    slide_moves(&state.board, from, state.to_move, &DIAGONALS)
}

// --- Rook --- //

fn rook_moves(state: &State, from: Square) -> impl Iterator<Item = Move> + '_ {
    slide_moves(&state.board, from, state.to_move, &ORTHOGONALS)
}

// --- Queen --- //

fn queen_moves(state: &State, from: Square) -> impl Iterator<Item = Move> + '_ {
    slide_moves(&state.board, from, state.to_move, &ALL_DIRECTIONS)
}

// --- King --- //

fn king_moves(state: &State, from: Square) -> impl Iterator<Item = Move> + '_ {
    step_moves(&state.board, from, state.to_move, &ALL_DIRECTIONS).chain(castling_moves(state))
}

fn castling_moves(state: &State) -> impl Iterator<Item = Move> + '_ {
    gen move {
        let color = state.to_move;
        let rank = color.home_rank();
        let from = Square::from_coords(rank, CastlingSide::KING_FILE);

        for side in CastlingSide::BOTH {
            if !state.castling_rights.has(color, side) {
                continue;
            }

            let clear = side
                .corridor_files()
                .iter()
                .all(|&f| state.board[Square::from_coords(rank, f)].is_none());
            if !clear {
                continue;
            }

            let safe = !is_square_attacked(&state.board, from, !color)
                && side.king_path_files().iter().all(|&f| {
                    !is_square_attacked(&state.board, Square::from_coords(rank, f), !color)
                });
            if !safe {
                continue;
            }

            yield Move::castling(from, Square::from_coords(rank, side.king_target_file()));
        }
    }
}

// --- Pawn --- //

fn pawn_moves(state: &State, from: Square) -> impl Iterator<Item = Move> + '_ {
    gen move {
        let color = state.to_move;
        let board = &state.board;

        // Single push
        if let Some(to) = from.forward(color, 1, Lateral::Straight)
            && board[to].is_none()
        {
            for mv in pawn_move_or_promote(from, to, color) {
                yield mv;
            }

            // Double push (only from starting rank, only if single push succeeded)
            if from.rank() == color.pawn_rank()
                && let Some(to2) = from.forward(color, 2, Lateral::Straight)
                && board[to2].is_none()
            {
                yield Move::new(from, to2);
            }
        }

        // Captures (normal and en passant)
        for lat in Lateral::CAPTURES {
            if let Some(to) = from.forward(color, 1, lat) {
                if let Some(target) = board[to] {
                    if target.color() != color {
                        for mv in pawn_move_or_promote(from, to, color) {
                            yield mv;
                        }
                    }
                } else if state.en_passant == Some(to) {
                    yield Move::en_passant(from, to);
                }
            }
        }
    }
}

// ============================================================================
// Legality Checking
// ============================================================================

fn is_legal(state: &State, mv: Move) -> bool {
    let new_state = state.clone().apply_move(mv);
    let king_sq = find_king(&new_state.board, state.to_move);
    !is_square_attacked(&new_state.board, king_sq, !state.to_move)
}

fn find_king(board: &Board, color: Color) -> Square {
    board
        .find(|p| p.is_king() && p.color() == color)
        .next()
        .expect("king must exist")
        .0
}

// ============================================================================
// Attack Detection
// ============================================================================

pub fn is_square_attacked(board: &Board, square: Square, by: Color) -> bool {
    // Knight attacks
    for (dr, df) in KNIGHT_OFFSETS {
        if let Some(sq) = square.offset(dr, df)
            && let Some(p) = board[sq]
            && p.color() == by
            && p.piece_type() == PieceType::Knight
        {
            return true;
        }
    }

    // King attacks
    for (dr, df) in ALL_DIRECTIONS {
        if let Some(sq) = square.offset(dr, df)
            && let Some(p) = board[sq]
            && p.color() == by
            && p.is_king()
        {
            return true;
        }
    }

    // Pawn attacks
    for lat in Lateral::CAPTURES {
        if let Some(sq) = square.forward(!by, 1, lat)
            && let Some(p) = board[sq]
            && p.color() == by
            && p.is_pawn()
        {
            return true;
        }
    }

    // Diagonal sliding attacks (bishop or queen)
    for (dr, df) in DIAGONALS {
        let mut sq = square;
        while let Some(next) = sq.offset(dr, df) {
            if let Some(p) = board[next] {
                if p.color() == by && matches!(p.piece_type(), PieceType::Bishop | PieceType::Queen)
                {
                    return true;
                }
                break;
            }
            sq = next;
        }
    }

    // Orthogonal sliding attacks (rook or queen)
    for (dr, df) in ORTHOGONALS {
        let mut sq = square;
        while let Some(next) = sq.offset(dr, df) {
            if let Some(p) = board[next] {
                if p.color() == by && matches!(p.piece_type(), PieceType::Rook | PieceType::Queen) {
                    return true;
                }
                break;
            }
            sq = next;
        }
    }

    false
}

pub fn is_in_check(state: &State) -> bool {
    let king_sq = find_king(&state.board, state.to_move);
    is_square_attacked(&state.board, king_sq, !state.to_move)
}
