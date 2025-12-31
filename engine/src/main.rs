use engine::board::*;

fn main() {
    let mut board = Board::new();

    let white_pawn = Piece::new(PieceType::Pawn, Color::White);
    let black_knight = Piece::new(PieceType::Knight, Color::Black);

    // Test set_piece on empty square
    let old = board.set_piece(white_pawn, (1, 4));  // e2
    println!("set_piece on empty: {}", old.is_none());  // Should be true

    // Test set_piece on occupied square (replace)
    let old = board.set_piece(black_knight, (1, 4));  // e2
    println!("set_piece on occupied: {} (was pawn: {})", old.is_some(),
        old.map(|p| p.piece_type() == PieceType::Pawn).unwrap_or(false));

    // Test remove_piece on occupied square
    let old = board.remove_piece((1, 4));  // e2
    println!("remove_piece on occupied: {} (was knight: {})", old.is_some(),
        old.map(|p| p.piece_type() == PieceType::Knight).unwrap_or(false));

    // Test remove_piece on empty square
    let old = board.remove_piece((1, 4));  // e2
    println!("remove_piece on empty: {}", old.is_none());  // Should be true

    // Setup for move_piece tests
    board.set_piece(white_pawn, (1, 4));  // white pawn on e2
    board.set_piece(black_knight, (3, 4));  // black knight on e4

    println!("\nBoard before moves:");
    println!("{}", board);

    // Test move_piece: occupied -> empty (no capture)
    let captured = board.move_piece((1, 4), (2, 4));  // e2 -> e3
    println!("move e2->e3 (no capture): captured={}", captured.is_none());  // true

    // Test move_piece: occupied -> occupied (capture)
    let captured = board.move_piece((2, 4), (3, 4));  // e3 -> e4 (captures knight)
    println!("move e3->e4 (capture): captured={}, was_knight={}",
        captured.is_some(),
        captured.map(|p| p.piece_type() == PieceType::Knight).unwrap_or(false));

    // Test move_piece: empty -> empty (no-op)
    let captured = board.move_piece((0, 0), (1, 1));  // empty squares
    println!("move empty->empty: captured={}", captured.is_none());  // true

    // Test move_piece: empty -> occupied (no-op, shouldn't capture!)
    board.set_piece(black_knight, (5, 5));  // f6
    let captured = board.move_piece((0, 0), (5, 5));  // empty -> f6
    println!("move empty->occupied: captured={} (should be false!)", captured.is_some());
    println!("piece still at f6: {}", board.piece_at((5, 5)).is_some());  // true

    println!("\nFinal board:");
    println!("{}", board);
}
