//! Chess game state.

use crate::board::{Board, Color, Lateral, Piece, Square, SquareExt};
use crate::castling::{CastlingRights, CastlingSide};
use crate::mobility::MoveGenerator;
use crate::mv::{Move, MoveType};

// ============================================================================
// Type Definitions
// ============================================================================

#[derive(Clone)]
pub struct State {
    pub(crate) board: Board,
    pub(crate) to_move: Color,
    pub(crate) castling_rights: CastlingRights,
    pub(crate) en_passant: Option<Square>,
    pub(crate) halfmove_clock: u8,
    pub(crate) fullmove_number: u16,
}

// ============================================================================
// State — Move Application
// ============================================================================

impl State {

    // --- Public Interface --- //
    pub fn apply_move(mut self, mv: Move) -> Self {
        let piece = self.board[mv.source()].unwrap();
        let captured = self.execute_board(mv);

        self.en_passant = self.resulting_en_passant(mv, piece);
        self.halfmove_clock = if piece.is_pawn() || captured.is_some() { 0 } else { self.halfmove_clock + 1 };
        self.castling_rights = self.resulting_castling(mv, piece, captured);
        self.fullmove_number += (self.to_move == Color::Black) as u16;
        self.to_move = !self.to_move;

        self
    }

    // --- Board Execution --- //
    fn execute_board(&mut self, mv: Move) -> Option<Piece> {
        match mv.move_type() {
            MoveType::Normal => self.board.move_piece(mv.source(), mv.target()),
            MoveType::Promotion => {
                self.board[mv.source()].lift();
                let promoted = Piece::new(mv.promotion_piece().unwrap(), self.to_move);
                self.board[mv.target()].place(promoted)
            }
            MoveType::EnPassant => {
                self.board.move_piece(mv.source(), mv.target());
                self.board[mv.en_passant_capture()].lift()
            }
            MoveType::Castling => {
                self.board.move_piece(mv.source(), mv.target());
                let (rf, rt) = mv.castling_rook_squares();
                self.board.move_piece(rf, rt)
            }
        }
    }

    // --- Derived State Computations --- //
    fn resulting_en_passant(&self, mv: Move, piece: Piece) -> Option<Square> {
        if !piece.is_pawn() { return None; }
        if (mv.target() - mv.source()).0.abs() != 2 { return None; }
        mv.source().forward(self.to_move, 1, Lateral::Straight)
    }

    fn resulting_castling(&self, mv: Move, piece: Piece, _captured: Option<Piece>) -> CastlingRights {
        if piece.is_king() { return self.castling_rights.lose_all(self.to_move); }
        
        let mut rights = self.castling_rights;
        
        // Rook moved from home square
        if let Some(side) = self.rook_home_side(mv.source(), self.to_move) {
            rights = rights.lose(self.to_move, side);
        }
        // Piece captured on opponent's home rook square
        if let Some(side) = self.rook_home_side(mv.target(), !self.to_move) {
            rights = rights.lose(!self.to_move, side);
        }
        
        rights
    }

    fn rook_home_side(&self, square: Square, color: Color) -> Option<CastlingSide> {
        let home_rank = if color == Color::White { 0 } else { 7 };
        if square.rank() != home_rank { return None; }
        CastlingSide::from_rook_file(square.file())
    }
}

// ============================================================================
// State — Move Generation
// ============================================================================

impl State {
    pub fn moves(&self) -> MoveGenerator<'_> {
        MoveGenerator::new(self)
    }
}
