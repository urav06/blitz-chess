use engine::board::{Board, Color, Piece, PieceType, SlotExt, Square};
use engine::castling::CastlingRights;
use engine::state::State;

fn main() {
    // Set up: White Ke1, Ng1, Pe2 — Black Ke8
    let mut board = Board::new();
    board[Square::from_coords(0, 4)].place(Piece::new(PieceType::King, Color::White));
    board[Square::from_coords(0, 6)].place(Piece::new(PieceType::Knight, Color::White));
    board[Square::from_coords(1, 4)].place(Piece::new(PieceType::Pawn, Color::White));
    board[Square::from_coords(7, 4)].place(Piece::new(PieceType::King, Color::Black));

    println!("{}\n", board);

    println!("Pieces:");
    for (sq, piece) in board.pieces() {
        println!("  {} at {}", piece, sq);
    }
    println!();

    let state = State::new(board, Color::White, CastlingRights::none());
    let moves: Vec<_> = state.moves().all().collect();

    println!("{} legal moves:", moves.len());
    for mv in &moves {
        println!("  {} → {}", mv.source(), mv.target());
    }
}
