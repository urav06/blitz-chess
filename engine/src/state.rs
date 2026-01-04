//! Chess game state.

use crate::board::{Board, Color, Lateral, Piece, PieceType, Square};
use crate::castling::CastlingRights;
use crate::mv::{Move, MoveType};

// ============================================================================
// Type Definitions
// ============================================================================

pub struct State {
    pub board: Board,
    pub to_move: Color,
    pub castling_rights: CastlingRights,
    pub en_passant: Option<Square>,
    pub halfmove_clock: u8,
    pub fullmove_number: u16,
}

// ============================================================================
// State — Move Application
// ============================================================================

impl State {

    // --- Public Interface --- //

    /// Apply a move, consuming the current state and returning the new state.
    pub fn apply_move(self, mv: Move) -> Self {
        match mv.move_type() {
            MoveType::Normal    => self.apply_normal(mv),
            MoveType::Promotion => self.apply_promotion(mv),
            MoveType::EnPassant => self.apply_en_passant(mv),
            MoveType::Castling  => self.apply_castling(mv),
        }
    }

    // --- Move Type Handlers --- //

    fn apply_normal(mut self, mv: Move) -> Self {
        let piece = self.board.piece_at(mv.source()).unwrap();
        let captured = self.board.move_piece(mv.source(), mv.target());
        let is_pawn = piece.piece_type() == PieceType::Pawn;

        State {
            to_move: !self.to_move,
            castling_rights: self.castling_rights,  // TODO: update
            en_passant: self.resulting_en_passant(mv, piece),
            halfmove_clock: if is_pawn || captured.is_some() { 0 } else { self.halfmove_clock + 1 },
            fullmove_number: self.fullmove_number + (self.to_move == Color::Black) as u16,
            ..self
        }
    }

    fn apply_promotion(mut self, mv: Move) -> Self {
        let _captured = self.board.remove_piece(mv.target());  // TODO: for castling rights
        self.board.remove_piece(mv.source());
        let promoted = Piece::new(mv.promotion_piece().unwrap(), self.to_move).with_moved();
        self.board.set_piece(promoted, mv.target());

        State {
            to_move: !self.to_move,
            castling_rights: self.castling_rights,  // TODO: update
            en_passant: None,
            halfmove_clock: 0,  // pawn move
            fullmove_number: self.fullmove_number + (self.to_move == Color::Black) as u16,
            ..self
        }
    }

    fn apply_en_passant(mut self, mv: Move) -> Self {
        self.board.move_piece(mv.source(), mv.target());
        self.board.remove_piece(mv.en_passant_capture());

        State {
            to_move: !self.to_move,
            castling_rights: self.castling_rights,
            en_passant: None,
            halfmove_clock: 0,  // pawn move + capture
            fullmove_number: self.fullmove_number + (self.to_move == Color::Black) as u16,
            ..self
        }
    }

    fn apply_castling(mut self, mv: Move) -> Self {
        self.board.move_piece(mv.source(), mv.target());
        let (rook_from, rook_to) = mv.castling_rook_squares();
        self.board.move_piece(rook_from, rook_to);

        State {
            to_move: !self.to_move,
            castling_rights: self.castling_rights.lose_all(self.to_move),
            en_passant: None,
            halfmove_clock: self.halfmove_clock + 1,
            fullmove_number: self.fullmove_number + (self.to_move == Color::Black) as u16,
            ..self
        }
    }

    // --- Helpers --- //

    fn resulting_en_passant(&self, mv: Move, piece: Piece) -> Option<Square> {
        if piece.piece_type() != PieceType::Pawn {
            return None;
        }

        let is_double_push = (mv.target().rank() as i8 - mv.source().rank() as i8).abs() == 2;
        if !is_double_push {
            return None;
        }

        // En passant square is the square the pawn skipped over
        mv.source().forward(self.to_move, 1, Lateral::Straight)
    }
}
