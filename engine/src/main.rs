use engine::board::*;

fn main() {
    let board = Board::new();

    // Test pieces
    let white_pawn = Piece::new(PieceType::Pawn, Color::White);
    let white_king = Piece::new(PieceType::King, Color::White);
    let black_queen = Piece::new(PieceType::Queen, Color::Black);
    let black_rook = Piece::new(PieceType::Rook, Color::Black);

    // Test Square display
    let e4 = Square::from_coords(3, 4);
    println!("Square display: {}", e4);

    // Test Piece display
    println!("White pawn: {}", white_pawn);
    println!("Black queen: {}", black_queen);

    // Test Board display - empty board
    println!("\nEmpty board:");
    println!("{}", board);

    // Test Board with pieces
    let board = board
        .with_piece(white_pawn, (1, 4))
        .with_piece(white_king, (0, 4))
        .with_piece(black_queen, (7, 3))
        .with_piece(black_rook, (7, 0));

    println!("\nBoard with pieces:");
    println!("{}", board);
}
