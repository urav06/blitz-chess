//! Chess game state.

use crate::board::{Board, Color, Lateral, Piece, Square, SlotExt};
use crate::castling::CastlingRights;
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

    // --- Move Application --- //
    pub fn apply_move(mut self, mv: Move) -> Self {
        let piece = self.board[mv.source()].unwrap();
        let captured = self.execute_board(mv);

        self.en_passant = self.resulting_en_passant(mv, piece);
        self.castling_rights = self.resulting_castling(mv, piece);
        self.halfmove_clock = self.resulting_halfmove(piece, captured.is_some());
        
        self.fullmove_number += (self.to_move == Color::Black) as u16;
        self.to_move = !self.to_move;

        self
    }

    fn execute_board(&mut self, mv: Move) -> Option<Piece> {
        match mv.move_type() {
            MoveType::Normal => self.board.move_piece(mv.source(), mv.target()),
            MoveType::Promotion => {
                self.board[mv.source()].lift();
                self.board[mv.target()].place(mv.promoted_piece(self.to_move))
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

    // --- State Derivations --- //
    fn resulting_en_passant(&self, mv: Move, piece: Piece) -> Option<Square> {
        if !piece.is_pawn() { return None; }
        if (mv.target() - mv.source()).0.abs() != 2 { return None; }
        mv.source().forward(self.to_move, 1, Lateral::Straight)
    }

    fn resulting_castling(&self, mv: Move, piece: Piece) -> CastlingRights {
        if piece.is_king() { return self.castling_rights.lose_all(self.to_move); }
        self.castling_rights
            .lose_for_rook_at(mv.source(), self.to_move)
            .lose_for_rook_at(mv.target(), !self.to_move)
    }

    fn resulting_halfmove(&self, piece: Piece, was_capture: bool) -> u8 {
        if piece.is_pawn() || was_capture { 0 } else { self.halfmove_clock + 1 }
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
